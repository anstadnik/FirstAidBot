mod bot;
use crate::bot::prelude::*;
use anyhow::{anyhow, Result};
use first_aid_bot_core::prelude::*;
use redis::aio::MultiplexedConnection;
use std::{borrow::Cow, sync::OnceLock};
pub use teloxide::types::UserId;

const MAINTAINER_IDS: [teloxide::types::UserId; 1] = [UserId(131_596_643)];
const MAINTAINER_USERNAMES: [&str; 2] = ["astadnik", "Oleksa_Lev"];
const REDIS_URLS: [&str; 2] = ["redis://redis:6379", "redis://127.0.0.1:6379"];
const REDIS_USERS_SET_KEY: &str = "all_users";
const BROADCAST_ENABLED: bool = false;

static DATA: OnceLock<Data> = OnceLock::new();
static REDIS_CONN: OnceLock<MultiplexedConnection> = OnceLock::new();

pub trait DataGetState {
    fn get_state(
        &self,
        ctx: &FAContext,
    ) -> impl std::future::Future<Output = Result<Cow<Fs>>> + Send;
}

impl DataGetState for OnceLock<Data> {
    async fn get_state(&self, ctx: &FAContext) -> Result<Cow<Fs>> {
        self.get()
            .ok_or_else(|| anyhow!("Data is not initialized"))?
            .get()
            .await?
            .get_state(ctx)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init_timed();
    let commit = option_env!("GITHUB_SHA").unwrap_or("unknown");
    log::info!("Starting bot on commit {commit} ...");
    let (redis_conn, storage, data) = init_connections().await?;
    DATA.set(data)
        .map_err(|_| anyhow!("OnceLock is already initialized"))?;
    REDIS_CONN
        .set(redis_conn)
        .map_err(|_| anyhow!("OnceLock is already initialized"))?;

    run_bot(storage).await
}
