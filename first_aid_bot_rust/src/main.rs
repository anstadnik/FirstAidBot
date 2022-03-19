mod bot;
mod lang;
mod model;

use crate::bot::run_bot;
use model::prelude::*;

const MAINTAINER_ID: i64 = 131596643;
const REDIS_KEY: &str = "user_ids";
const SHEET_ID: &str = "Миші з'їли";

#[tokio::main]
async fn main() {
    let data = if cfg!(debug_assertions) {
        Data::dynamic()
    } else {
        Data::cached().await
    };

    run_bot(data).await;
}
