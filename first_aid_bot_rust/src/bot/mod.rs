mod commands;
mod dialogue;
mod helpers;

use crate::bot::commands::{get_commands_branch, get_maintainer_commands_branch, FirstAidCommands};
use crate::bot::dialogue::State;
use crate::model::prelude::*;
use futures::future::join_all;
use redis::{aio::MultiplexedConnection, Client};
use std::sync::Arc;
use teloxide::dispatching2::dialogue::{serializer::Bincode, RedisStorage};
use teloxide::{prelude2::*, utils::command::BotCommand};

async fn try_connect(
    url: &str,
) -> anyhow::Result<(MultiplexedConnection, Arc<RedisStorage<Bincode>>)> {
    anyhow::Ok((
        Client::open(url)?
            .get_multiplexed_tokio_connection()
            .await?,
        RedisStorage::open(url, Bincode).await?,
    ))
}

pub async fn run_bot(data: Data) {
    teloxide::enable_logging!();
    log::info!("Starting dialogue_bot...");

    let bot = Bot::from_env()
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .auto_send();

    bot.set_my_commands(FirstAidCommands::bot_commands())
        .await
        .unwrap();

    let urls = vec!["redis://redis:6379", "redis://127.0.0.1:6379"];

    let (redis_con, storage) = join_all(urls.into_iter().map(try_connect))
        .await
        .into_iter()
        .find_map(Result::ok)
        .unwrap();

    let handler = Update::filter_message()
        .branch(get_commands_branch())
        .branch(get_maintainer_commands_branch())
        .enter_dialogue::<Message, RedisStorage<Bincode>, State>()
        .dispatch_by::<State>();

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![Arc::new(data), redis_con, storage])
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}
