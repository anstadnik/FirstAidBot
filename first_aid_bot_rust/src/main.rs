mod bot;
mod lang;
mod model;

use crate::bot::run_bot;
use model::prelude::*;

const MAINTAINER_ID: i64 = 131596643;
const REDIS_KEY: &str = "user_ids";
const SHEET_ID: &str = "1cO0sPRhIvt71J-iB313BeRfNXzXM0FjiQ4bDYmwddBQ";

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
