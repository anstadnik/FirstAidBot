mod bot;
mod model;

use crate::bot::run_bot;
use model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, std::hash::Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Lang {
    name: &'static str,
    text: &'static str,
    sheet: &'static str,
    greet: &'static str,
}

impl Lang {
    const fn new(
        name: &'static str,
        text: &'static str,
        sheet: &'static str,
        greet: &'static str,
    ) -> Self {
        Self {
            name,
            text,
            sheet,
            greet,
        }
    }
}

const MAINTAINER_ID: i64 = 131596643;
const REDIS_KEY: &str = "user_ids";
const SHEET_ID: &str = "Миші з'їли";

pub const LANGS: [Lang; 2] = [
    Lang::new("Ukrainian", "Українська", "Ukrainian", "Що трапилось?"),
    Lang::new("English", "English", "English", "What happened?"),
];

fn main() {
    let data = Data::dynamic();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run_bot(data))
}
