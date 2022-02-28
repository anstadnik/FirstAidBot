use std::sync::Arc;
use clap::Parser;
use first_aid_bot_rust::{run_bot, get_data};

// Prod: "1cO0sPRhIvt71J-iB313BeRfNXzXM0FjiQ4bDYmwddBQ";
// Test: "1seobblWaZXSu82yf3CnanIps26vCv3QARo75-sAC2KQ";
static SHEET_ID_DEFAULT: &str = "1seobblWaZXSu82yf3CnanIps26vCv3QARo75-sAC2KQ";
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap( short, long, default_value = SHEET_ID_DEFAULT
    )]
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
        .block_on(run_bot(Arc::new(data)))
}
