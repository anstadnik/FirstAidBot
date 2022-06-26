use crate::REDIS_URLS;
use futures::future::join_all;
use redis::{aio::MultiplexedConnection, Client};
use teloxide::dispatching::dialogue::{serializer::Bincode, RedisStorage};
use teloxide::types::{ChatId, ParseMode};
use teloxide::{payloads::SendMessageSetters, prelude::*};

use super::prelude::*;

pub async fn send_message(bot: &FirstAirBot, id: ChatId, msg: String) -> anyhow::Result<()> {
    #[allow(deprecated)]
    bot.send_message(id, msg)
        .parse_mode(ParseMode::Markdown)
        .await?;
    Ok(())
}

pub async fn connect_to_redis() -> (MultiplexedConnection, Arc<FirstAidStorage>) {
    let results = join_all(REDIS_URLS.into_iter().map(|url| async move {
        let connection = Client::open(url)?
            .get_multiplexed_tokio_connection()
            .await?;
        anyhow::Ok((connection, RedisStorage::open(url, Bincode).await?))
    }));
    results.await.into_iter().find_map(Result::ok).unwrap()
}
