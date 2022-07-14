use super::prelude::*;
use crate::MAINTAINER_IDS;
use futures::future::BoxFuture;
use itertools::Itertools;
use std::fmt::Display;
use std::sync::Arc;
use teloxide::error_handlers::ErrorHandler;
use teloxide::utils::markdown::escape;

fn split_msg(msg: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    for msg in msg.split_inclusive('\n') {
        if ret.is_empty() || ret.last().unwrap().len() + msg.len() >= 4000 {
            ret.push(msg.to_string());
        } else {
            *ret.last_mut().unwrap() += msg;
        }
    }
    ret.into_iter()
        .flat_map(|msg: String| -> Vec<String> {
            msg.chars()
                .chunks(4000)
                .into_iter()
                .map(|c| c.collect())
                .collect()
        })
        .collect()
}

pub async fn send_plain_string(bot: &FABot, id: ChatId, msg: &str) -> anyhow::Result<()> {
    for msg in &split_msg(msg) {
        bot.send_message(id, "```".to_string() + &escape(msg) + "```")
            .await?;
    }
    Ok(())
}

pub async fn send_err(bot: &FABot, id: ChatId, lang: &Lang, err: impl Display) {
    if let Err(err) = async move {
        send_plain_string(bot, id, lang.details().error).await?;
        send_plain_string(bot, id, &err.to_string()).await?;
        anyhow::Ok(())
    }
    .await
    {
        log::error!("OH MY GOD SOMETHING BROKEN AND I CAN'T EVEN REPORT IT");
        log::error!("The sending error is:");
        log::error!("{err}");
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

impl<E> ErrorHandler<E> for FAErrorHandler
where
    E: Display + Sync + Send,
{
    fn handle_error(self: Arc<Self>, err: E) -> BoxFuture<'static, ()> {
        log::error!("{}", err.to_string());
        Box::pin(async move {
            if !cfg!(debug_assertions) {
                for &maintainer_id in &MAINTAINER_IDS {
                    send_err(&self.bot, maintainer_id.into(), &Default::default(), err).await;
                }
            }
        })
    }
}
