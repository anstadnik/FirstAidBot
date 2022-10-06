use crate::bot::prelude::*;
use anyhow::{Error, Result};
use futures::{future::BoxFuture, FutureExt};
use itertools::Itertools;
use teloxide::utils::markdown::code_block;

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
    ret.into_iter().flat_map(|msg| -> Vec<String> {
        msg.chars()
            .chunks(4000)
            .into_iter()
            .map(Iterator::collect)
            .collect()
    })
}

async fn send_escaped(bot: &FABot, id: ChatId, msg: &str) -> anyhow::Result<()> {
    for msg in split_msg(msg) {
        bot.send_message(id, code_block(&msg)).await?;
    }
    Ok(())
}

pub trait ReportError {
    fn report_if_err<'a>(
        self,
        bot: &'a FABot,
        id: ChatId,
        lang: &'a Lang,
        msg: Option<&'a str>,
    ) -> BoxFuture<'a, Self>;
}

impl<T> ReportError for Result<T>
where
    for<'a> T: Send + Sync + 'a,
{
    fn report_if_err<'a>(
        self,
        bot: &'a FABot,
        id: ChatId,
        lang: &'a Lang,
        msg: Option<&'a str>,
    ) -> BoxFuture<'a, Self> {
        async move {
            if let Err(err) = &self {
                report_error(bot, id, lang, msg, err).await
            }
            self
        }
        .boxed()
    }
}

pub async fn report_error(bot: &FABot, id: ChatId, lang: &Lang, msg: Option<&str>, err: &Error) {
    if let Err(err) = async {
        send_escaped(bot, id, msg.unwrap_or(lang.details().error)).await?;
        send_escaped(bot, id, &format!("{err:?}")).await
    }
    .await
    {
        log::error!("OH MY GOD SOMETHING BROKEN AND I CAN'T EVEN REPORT IT");
        log::error!("The sending error is: {err}");
    }
}
