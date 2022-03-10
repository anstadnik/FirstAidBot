mod bot;
mod model;

use crate::bot::run_bot;
use crate::model::get_data;
use clap::Parser;

#[derive(
    Clone, std::hash::Hash, std::cmp::Eq, std::cmp::PartialEq, serde::Serialize, serde::Deserialize,
)]
pub struct Lang {
    name: &'static str,
    text: &'static str,
    sheet: &'static str,
}

impl Lang {
    const fn new(name: &'static str, text: &'static str, sheet: &'static str) -> Self {
        Self { name, text, sheet }
    }
}

const MAINTAINER_ID: i64 = 131596643;
const REDIS_KEY: &str = "user_ids";
const SHEET_ID: &str = "1cO0sPRhIvt71J-iB313BeRfNXzXM0FjiQ4bDYmwddBQ";

pub const LANGS: [Lang; 2] = [
    Lang::new("Ukrainian", "Ukrainian", "Ukrainian"),
    Lang::new("English", "English", "English"),
];

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap( short, long, default_value = SHEET_ID)]
    pub sheet_id: String,
}

fn main() {
    let args = Args::parse();
    let data = get_data(args.sheet_id.as_str());

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run_bot(data))
}
