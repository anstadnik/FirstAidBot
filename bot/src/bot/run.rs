use crate::bot::dialogue::prelude::*;
use crate::bot::report_error::FAErrorHandler;
use crate::bot::State;
use crate::REDIS_URLS;
use anyhow::{anyhow, Result};
use first_aid_bot_core::prelude::Data;
use futures::future::join_all;
use redis::{aio::MultiplexedConnection, Client};
use std::sync::Arc;
use teloxide::dispatching::dialogue::{serializer::Bincode, RedisStorage};
use teloxide::dptree::case;
use teloxide::prelude::*;
use teloxide::{adaptors::throttle::Limits, types::ParseMode, utils::command::BotCommands};

use super::FirstAidStorage;

pub async fn connect_to_redis() -> Result<(MultiplexedConnection, Arc<FirstAidStorage>)> {
    let results = join_all(REDIS_URLS.into_iter().map(|url| async move {
        let connection = Client::open(url)?
            .get_multiplexed_tokio_connection()
            .await?;
        anyhow::Ok((connection, RedisStorage::open(url, Bincode).await?))
    }));
    let error = anyhow!("No redis connection");
    results.await.into_iter().flatten().next().ok_or(error)
}

pub async fn run_bot(data: &'static Data) -> Result<()> {
    log::info!("Starting dialogue_bot...");

    let bot = Bot::from_env()
        .throttle(Limits::default())
        .parse_mode(ParseMode::MarkdownV2);

    bot.set_my_commands(FACommands::bot_commands()).await?;

    let (conn, storage) = connect_to_redis().await?;

    let handler = Update::filter_message()
        .branch(get_commands_branch())
        .branch(get_maintainer_commands_branch())
        .enter_dialogue::<Message, FirstAidStorage, State>()
        .branch(case![State::Start].endpoint(start_endpoint))
        .branch(case![State::Dialogue { lang, context }].endpoint(handle_endpoint))
        .branch(case![State::Broadcast { message }].endpoint(broadcast_endpoint));

    Dispatcher::builder(bot.clone(), handler)
        .dependencies(dptree::deps![data, conn, storage])
        .error_handler(FAErrorHandler::new(bot))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}
