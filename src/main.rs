#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod bot;
mod model;

use bot::run_bot;
use model::prelude::Data;
use teloxide::types::UserId;

const MAINTAINER_IDS: [teloxide::types::UserId; 1] = [UserId(131_596_643)];
const MAINTAINER_USERNAMES: [&str; 2] = ["astadnik", "Oleksa_Lev"];
const HELP_CHAT_URL: &str = "https://t.me/+VOd-pnzhWvU2Yjg6";
const REDIS_URLS: [&str; 2] = ["redis://redis:6379", "redis://127.0.0.1:6379"];
const REDIS_USERS_SET_KEY: &str = "all_users";
const BROADCAST_ENABLED: bool = false;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let data = if cfg!(debug_assertions) {
        Data::dynamic()
    } else {
        Data::cached().await
    };

    run_bot(data).await;
}
