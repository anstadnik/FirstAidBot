use crate::MAINTAINER_IDS;
use anyhow::Error;
use first_aid_bot_core::prelude::Lang;
use futures::future::BoxFuture;
use itertools::Itertools;
use std::sync::Arc;
use teloxide::error_handlers::ErrorHandler;
use teloxide::prelude::*;
use teloxide::utils::markdown::code_block;

use super::FABot;

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

pub async fn report_error(bot: &FABot, id: ChatId, msg: &str, err: &Error) {
    if let Err(err) = async {
        send_escaped(bot, id, msg).await?;
        send_escaped(bot, id, &format!("{err:?}")).await
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
        log::error!("{err:?}");
        Box::pin(async move {
            if !cfg!(debug_assertions) {
                for &id in &MAINTAINER_IDS {
                    report_error(&self.bot, id.into(), Lang::default().details().error, &err).await;
                }
            }
        })
    }
}
