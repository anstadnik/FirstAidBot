mod commands;
mod dialogue;
mod helpers;

use crate::bot::commands::{get_commands_branch, get_maintainer_commands_branch, FirstAidCommands};
use crate::bot::dialogue::{handle_dialogue, reset_dialogue, State};
use crate::model::prelude::*;
use futures::future::join_all;
use redis::{aio::MultiplexedConnection, Client};
use teloxide::adaptors::throttle::Limits;
use std::sync::Arc;
use teloxide::dispatching::dialogue::{serializer::Bincode, RedisStorage};
use teloxide::types::ParseMode;
use teloxide::{prelude::*, utils::command::BotCommands};

async fn try_connect(
    url: &str,
) -> anyhow::Result<(MultiplexedConnection, Arc<RedisStorage<Bincode>>)> {
    let con = Client::open(url)?
        .get_multiplexed_tokio_connection()
        .await?;
    let storage = RedisStorage::open(url, Bincode).await?;
    anyhow::Ok((con, storage))
}

pub async fn run_bot(data: Data) {
    log::info!("Starting dialogue_bot...");

    let bot = Bot::from_env()
        .throttle(Limits::default())
        .parse_mode(ParseMode::MarkdownV2)
        .auto_send();

    bot.set_my_commands(FirstAidCommands::bot_commands())
        .await
        .unwrap();

    let urls = vec!["redis://redis:6379", "redis://127.0.0.1:6379"];

    let redis_results = join_all(urls.into_iter().map(try_connect)).await;
    let (redis_con, storage) = redis_results.into_iter().find_map(Result::ok).unwrap();

    let handler = Update::filter_message()
        .branch(get_commands_branch())
        .branch(get_maintainer_commands_branch())
        .enter_dialogue::<Message, RedisStorage<Bincode>, State>()
        .branch(teloxide::handler![State::Start { lang }].endpoint(reset_dialogue))
        .branch(teloxide::handler![State::Dialogue { lang, context }].endpoint(handle_dialogue));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![Arc::new(data), redis_con, storage])
        .error_handler(LoggingErrorHandler::new())
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}
