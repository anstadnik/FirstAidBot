use crate::bot::prelude::*;
use async_trait::async_trait;
use itertools::Itertools;
use teloxide::utils::markdown::code_block;

#[async_trait]
pub trait ReportError {
    async fn report_if_err(self, bot: &FABot, id: ChatId, lang: &Lang) -> Self;
}

#[async_trait]
impl<T> ReportError for anyhow::Result<T>
where
    T: std::marker::Send + std::marker::Sync,
{
    async fn report_if_err(self, bot: &FABot, id: ChatId, lang: &Lang) -> Self {
        if let Err(err) = &self {
            report_error(bot, id, lang, err).await
        }
        self
    }
}

pub async fn report_error(bot: &FABot, id: ChatId, lang: &Lang, err: &anyhow::Error) {
    if let Err(err) = async {
        send_escaped(bot, id, lang.details().error).await?;
        send_escaped(bot, id, &err.to_string()).await
    }
    .await
    {
        log::error!("OH MY GOD SOMETHING BROKEN AND I CAN'T EVEN REPORT IT");
        log::error!("The sending error is: {err}");
    }
}

// Waiting for https://github.com/teloxide/teloxide/issues/482
fn split_msg(msg: &str) -> impl Iterator<Item = String> {
    let mut ret: Vec<String> = Vec::new();
    for msg in msg.split_inclusive('\n') {
        if ret.is_empty() || ret.last().unwrap().len() + msg.len() >= 4000 {
            ret.push(msg.to_string());
        } else {
            *ret.last_mut().unwrap() += msg;
        }
    }
    ret.into_iter().flat_map(|msg: String| -> Vec<_> {
        msg.chars()
            .chunks(4000)
            .into_iter()
            .map(|c| c.collect())
            .collect()
    })
}

async fn send_escaped(bot: &FABot, id: ChatId, msg: &str) -> anyhow::Result<()> {
    for msg in split_msg(msg) {
        bot.send_message(id, code_block(&msg)).await?;
    }
    Ok(())
}
