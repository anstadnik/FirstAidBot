use first_aid_bot_core::prelude::Lang;
use std::sync::Arc;

use crate::{bot::report_error::report_error, MAINTAINER_IDS};
use futures::future::BoxFuture;
use teloxide::error_handlers::ErrorHandler;

use super::FABot;

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