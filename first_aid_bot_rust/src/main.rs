mod bot;
mod model;

use crate::bot::run_bot;
use model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, std::hash::Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Lang {
    name: &'static str,
    text: &'static str,
    greet: &'static str,
}

impl Lang {
    const fn new(name: &'static str, text: &'static str, greet: &'static str) -> Self {
        Self { name, text, greet }
    }
}

const MAINTAINER_ID: i64 = 131596643;
const REDIS_KEY: &str = "user_ids";
const SHEET_ID: &str = "Миші з'їли";

pub const LANGS: [Lang; 3] = [
    Lang::new("Ukrainian", "Українська", "Що трапилось?"),
    Lang::new("Russian", "Русский", "Что произошло?"),
    Lang::new("English", "English", "What happened?"),
];

#[tokio::main]
async fn main() {
    let data = if cfg!(debug_assertions) {
        Data::dynamic()
    } else {
        Data::cached().await
    };

    run_bot(data).await;
}
