use super::prelude::*;
use crate::MAINTAINER_IDS;
use futures::future::BoxFuture;
use std::fmt::Display;
use std::sync::Arc;
use teloxide::error_handlers::ErrorHandler;

pub async fn send_err(bot: &FABot, id: ChatId, lang: &Lang, err: String) {
    if let Err(err) = async move {
        send_plain_string(bot, id, lang.details().error).await?;
        send_plain_string(bot, id, &err).await?;
        anyhow::Ok(())
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

impl<E> ErrorHandler<E> for FAErrorHandler
where
    E: Display + Sync + Send,
{
    fn handle_error(self: Arc<Self>, err: E) -> BoxFuture<'static, ()> {
        log::error!("{}", err.to_string());
        let err = err.to_string();
        Box::pin(async move {
            if !cfg!(debug_assertions) {
                for &id in &MAINTAINER_IDS {
                    send_err(&self.bot, id.into(), &Default::default(), err.clone()).await;
                }
            }
        })
    }
}
