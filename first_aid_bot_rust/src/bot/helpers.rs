use super::prelude::*;
use crate::REDIS_URLS;
use futures::future::join_all;
use itertools::Itertools;
use redis::{aio::MultiplexedConnection, Client};
use teloxide::dispatching::dialogue::{serializer::Bincode, RedisStorage};
use teloxide::utils::markdown::escape;

fn split_msg(msg: &str) -> impl Iterator<Item = String> {
    let mut ret: Vec<String> = Vec::new();
    for msg in msg.split_inclusive('\n') {
        if ret.is_empty() || ret.last().unwrap().len() + msg.len() >= 4000 {
            ret.push(msg.to_string());
        } else {
            *ret.last_mut().unwrap() += msg;
        }
    }
    ret.into_iter().flat_map(|msg: String| -> Vec<_> {
        msg.chars()
            .chunks(4000)
            .into_iter()
            .map(|c| c.collect())
            .collect()
    })
}

pub async fn send_plain_string(bot: &FABot, id: ChatId, msg: &str) -> anyhow::Result<()> {
    for msg in split_msg(msg) {
        let msg = "```".to_string() + &escape(&msg) + "```";
        bot.send_message(id, msg).await?;
    }
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
