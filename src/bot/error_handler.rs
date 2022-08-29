use super::prelude::*;
use crate::MAINTAINER_IDS;
use anyhow::Result;
use futures::future::BoxFuture;
use std::sync::Arc;
use teloxide::error_handlers::ErrorHandler;

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
                    send_err(&self.bot, id.into(), &Default::default(), &err).await;
                }
            }
        })
    }
}
