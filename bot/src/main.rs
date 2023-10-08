mod bot;
use crate::bot::prelude::*;
use anyhow::{anyhow, Result};
use first_aid_bot_core::prelude::*;
use std::sync::OnceLock;

pub use teloxide::types::UserId;
const MAINTAINER_IDS: [teloxide::types::UserId; 1] = [UserId(131_596_643)];
const MAINTAINER_USERNAMES: [&str; 2] = ["astadnik", "Oleksa_Lev"];
const REDIS_URLS: [&str; 2] = ["redis://redis:6379", "redis://127.0.0.1:6379"];
const REDIS_USERS_SET_KEY: &str = "all_users";
const BROADCAST_ENABLED: bool = false;

static DATA: OnceLock<Data> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init_timed();
    let commit = option_env!("GITHUB_SHA").unwrap_or("unknown");
    log::info!("Starting bot on commit {commit} ...");
    let data = if cfg!(debug_assertions) {
        Data::dynamic()
    } else {
        Data::cached()?
    };
    DATA.set(data).map_err(|_| anyhow!("OnceLock is already initialized"))?;

    run_bot(DATA.get().ok_or(anyhow!("OnceLock is not initialized"))?).await
}
