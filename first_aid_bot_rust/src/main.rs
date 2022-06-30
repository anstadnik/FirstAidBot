mod bot;
mod model;

use bot::run_bot;
use log::info;
use model::prelude::*;
use teloxide::types::UserId;
// use teloxide::types::UserId;

const MAINTAINER_IDS: [teloxide::types::UserId; 1] = [UserId(131596643)];
const MAINTAINER_USERNAMES: [&str; 2] = ["astadnik", "AL_Lev"];
const HELP_CHAT_URL: &str = "https://t.me/+VOd-pnzhWvU2Yjg6";
const REDIS_URLS: [&str; 2] = ["redis://redis:6379", "redis://127.0.0.1:6379"];
const REDIS_USERS_SET_KEY: &str = "all_users";

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let data = if cfg!(debug_assertions) {
        info!("Dynamic data!");
        Data::dynamic()
    } else {
        info!("Cached data!");
        Data::cached().await
    };

    run_bot(data).await;
}
