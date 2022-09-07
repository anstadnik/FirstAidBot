use super::{dialogue::prelude::*, prelude::*};
use crate::{bot::error_handler::FAErrorHandler, REDIS_URLS};
use futures::future::join_all;
use redis::{aio::MultiplexedConnection, Client};
use teloxide::{
    adaptors::throttle::Limits,
    dispatching::dialogue::{serializer::Bincode, RedisStorage},
    types::ParseMode,
    utils::command::BotCommands,
};

pub async fn connect_to_redis() -> (MultiplexedConnection, Arc<FirstAidStorage>) {
    let results = join_all(REDIS_URLS.into_iter().map(|url| async move {
        let connection = Client::open(url)?
            .get_multiplexed_tokio_connection()
            .await?;
        anyhow::Ok((connection, RedisStorage::open(url, Bincode).await?))
    }));
    results.await.into_iter().find_map(Result::ok).unwrap()
}

pub async fn run_bot(data: Data) {
    log::info!("Starting dialogue_bot...");

    let bot = Bot::from_env()
        .throttle(Limits::default())
        .parse_mode(ParseMode::MarkdownV2)
        .auto_send();

    bot.set_my_commands(FACommands::bot_commands())
        .await
        .unwrap();

    let (redis_con, storage) = connect_to_redis().await;

    let handler = Update::filter_message()
        .branch(get_commands_branch())
        .branch(get_maintainer_commands_branch())
        .enter_dialogue::<Message, FirstAidStorage, State>()
        .branch(dptree::case![State::Start { lang }].endpoint(start_handler))
        .branch(dptree::case![State::Dialogue { lang, context }].endpoint(handle_dialogue));

    Dispatcher::builder(bot.clone(), handler)
        .dependencies(dptree::deps![Arc::new(data), redis_con, storage])
        .error_handler(FAErrorHandler::new(bot))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
