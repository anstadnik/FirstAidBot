mod bot;
mod lang;
mod model;

use crate::bot::run_bot;
use log::info;
use model::prelude::*;
use teloxide::types::UserId;

const MAINTAINER_ID: UserId = UserId(131596643);
const REDIS_KEY: &str = "user_ids";
const SHEET_ID: &str = "Миші з'їли";
const HELP_CHAT_URL: &str = "https://t.me/+VOd-pnzhWvU2Yjg6";

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
