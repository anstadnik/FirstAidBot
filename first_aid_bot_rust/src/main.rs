mod bot;
mod model;

use crate::bot::run_bot;
use crate::model::get_data;
use clap::Parser;
use std::sync::Arc;

// Prod
static SHEET_ID_PROD: &str = "1seobblWaZXSu82yf3CnanIps26vCv3QARo75-sAC2KQ";
// Test
static SHEET_ID_TEST: &str = "1cO0sPRhIvt71J-iB313BeRfNXzXM0FjiQ4bDYmwddBQ";
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap( short, long, default_value = SHEET_ID_TEST)]
    pub sheet_id: String,
}

fn main() {
    let args = Args::parse();
    let sheet_name = "Sheet1";
    let data = get_data(args.sheet_id.as_str(), sheet_name);
    log::debug!("Starting server");

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run_bot(Arc::new(data)))
}
