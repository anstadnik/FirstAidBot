mod commands;
mod dialogue;
mod helpers;

use crate::{
    bot::{
        commands::{
            commands_handler, maintainer_commands_handler, FirstAidCommands, MaintainerCommands,
        },
        dialogue::State,
    },
    model::FiniteState,
    MAINTAINER_ID,
};
use futures::future::join_all;
use redis::{aio::MultiplexedConnection, Client};
use std::sync::Arc;
use teloxide::{
    dispatching2::dialogue::{serializer::Bincode, RedisStorage},
    prelude2::*,
    utils::command::BotCommand,
};

pub async fn run_bot(data: FiniteState) {
    teloxide::enable_logging!();
    log::info!("Starting dialogue_bot...");

    let bot = Bot::from_env().auto_send();

    bot.set_my_commands(FirstAidCommands::bot_commands())
        .await
        .unwrap();

    let possible_redis_urls = vec!["redis://redis:6379", "redis://127.0.0.1:6379"];
    // Oh my god
    let (redis_con, storage) = join_all(possible_redis_urls.into_iter().map(|url| async move {
        Ok::<_, anyhow::Error>((
            Client::open(url)?
                .get_multiplexed_tokio_connection()
                .await?,
            RedisStorage::open(url, Bincode).await?,
        ))
    }))
    .await
    .into_iter()
    .find(Result::is_ok)
    .unwrap()
    .unwrap();

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<FirstAidCommands>()
                .enter_dialogue::<Message, RedisStorage<Bincode>, State>()
                .endpoint(commands_handler),
        )
        .branch(
            dptree::filter(
                |msg: Message,
                 _bot: AutoSend<Bot>,
                 _data: Arc<FiniteState>,
                 _redis_con: MultiplexedConnection| {
                    msg.from()
                        .map(|user| user.id == MAINTAINER_ID)
                        .unwrap_or_default()
                },
            )
            .filter_command::<MaintainerCommands>()
            .endpoint(maintainer_commands_handler),
        )
        .enter_dialogue::<Message, RedisStorage<Bincode>, State>()
        .dispatch_by::<State>();

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![Arc::new(data), redis_con, storage])
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}
