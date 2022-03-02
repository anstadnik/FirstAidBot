mod bot;
mod model;

use crate::bot::run_bot;
use crate::model::get_data;
use clap::Parser;

const MAINTAINER_ID: i64 = 131596643;
const REDIS_KEY: &str = "user_ids";
const SHEET_ID: &str = "Миші з'їли";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap( short, long, default_value = SHEET_ID)]
    pub sheet_id: String,
}

fn main() {
    let args = Args::parse();
    let sheet_name = "Sheet1";
    let data = get_data(args.sheet_id.as_str(), sheet_name);

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run_bot(data))
}
