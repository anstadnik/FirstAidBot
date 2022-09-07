use super::prelude::*;
use crate::MAINTAINER_IDS;
use anyhow::Result;
use futures::future::BoxFuture;
use itertools::Itertools;
use std::sync::Arc;
use teloxide::error_handlers::ErrorHandler;
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
    ret.into_iter().flat_map(|msg: String| -> Vec<_> {
        msg.chars()
            .chunks(4000)
            .into_iter()
            .map(|c| c.collect())
            .collect()
    })
}

pub async fn send_plain_string(bot: &FABot, id: ChatId, msg: &str) -> anyhow::Result<()> {
    for msg in split_msg(msg) {
        bot.send_message(id, code_block(&msg)).await?;
    }
    Ok(())
}

pub async fn report_if_error<T>(bot: &FABot, id: ChatId, lang: &Lang, rez: Result<T>) -> Result<T>
where
    T: Send + Sync,
{
    if let Err(err) = &rez {
        send_err(bot, id, lang, &format!("{err:?}")).await;
    }
    rez
}

pub async fn send_err(bot: &FABot, id: ChatId, lang: &Lang, err: &str) {
    if let Err(err) = async move {
        send_plain_string(bot, id, lang.details().error).await?;
        send_plain_string(bot, id, err).await
    }
    .await
    {
        log::error!("OH MY GOD SOMETHING BROKEN AND I CAN'T EVEN REPORT IT");
        log::error!("The sending error is: {err}");
    }
}

pub struct FAErrorHandler {
    bot: FABot,
}

impl FAErrorHandler {
    pub fn new(bot: FABot) -> Arc<Self> {
        Arc::new(Self { bot })
    }
}

impl ErrorHandler<anyhow::Error> for FAErrorHandler {
    fn handle_error(self: Arc<Self>, err: anyhow::Error) -> BoxFuture<'static, ()> {
        let err = format!("{err:?}");
        log::error!("{err}");
        Box::pin(async move {
            if !cfg!(debug_assertions) {
                for &id in &MAINTAINER_IDS {
                    let _ = send_plain_string(&self.bot, id.into(), "У когось біда!").await;
                    send_err(&self.bot, id.into(), &Lang::default(), &err).await;
                }
            }
        })
    }
}
