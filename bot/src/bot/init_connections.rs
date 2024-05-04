use std::sync::Arc;

use crate::REDIS_URLS;
use anyhow::{anyhow, Result};
use first_aid_bot_core::prelude::Data;
use futures::future::join_all;
use redis::{aio::MultiplexedConnection, Client};
use teloxide::dispatching::dialogue::{serializer::Bincode, RedisStorage};

async fn connect_to_redis() -> Result<(MultiplexedConnection, Arc<RedisStorage<Bincode>>)> {
    join_all(REDIS_URLS.into_iter().map(|url| async move {
        let connection = Client::open(url)?.get_multiplexed_tokio_connection().await;
        anyhow::Ok((connection?, RedisStorage::open(url, Bincode).await?))
    }))
    .await
    .into_iter()
    .flatten()
    .next()
    .ok_or(anyhow!("No redis connection"))
}

async fn init_data() -> Result<Data> {
    if cfg!(debug_assertions) {
        Ok(Data::dynamic())
    } else {
        Data::cached()
    }
}

pub async fn init_connections() -> Result<(MultiplexedConnection, Arc<RedisStorage<Bincode>>, Data)>
{
    let (redis_conn, storage) = connect_to_redis().await?;
    let data = init_data().await?;
    Ok((redis_conn, storage, data))
}
