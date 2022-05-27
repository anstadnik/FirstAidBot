use super::prelude::*;
use crate::MAINTAINER_ID;
use futures::future::BoxFuture;
use std::{fmt::Debug, sync::Arc};
use teloxide::error_handlers::ErrorHandler;

pub struct FirstAidErrorHandler {
    bot: FirstAidBot,
}

impl FirstAidErrorHandler {
    pub fn new(bot: FirstAidBot) -> Arc<Self> {
        Arc::new(Self { bot })
    }
}

impl<E> ErrorHandler<E> for FirstAidErrorHandler
where
    E: Debug,
{
    fn handle_error(self: Arc<Self>, error: E) -> BoxFuture<'static, ()> {
        let err = format!("{:?}", error);
        log::error!("{:?}", err);
        #[allow(deprecated)]
        Box::pin(async move {
            if send_message(&self.bot, MAINTAINER_ID.into(), err.clone()).await.is_err() {
                log::error!("OH MY GOD SOMETHING BROKE AND I CAN'T EVEN REPORT IT");
                log::error!("{}", err);
            };
        })
    }
}
