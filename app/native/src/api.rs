#![allow(dead_code)]
use anyhow::Result;
pub use first_aid_bot_core::prelude::*;
use flutter_rust_bridge::{RustOpaque, SyncReturn};
pub use std::sync::RwLock;

#[tokio::main]
pub async fn get_data() -> Result<RustOpaque<MultilangFs>> {
    get_data_from_web().await.map(RustOpaque::new)
    // get_data_from_file("/Users/astadnik/Ukraine/FirstAidBot/table.csv").map(RustOpaque::new)
}

type RWCTX = RustOpaque<RwLock<FAContext>>;
pub fn get_context() -> SyncReturn<RWCTX> {
    assert!(Lang::iter().count() == 1);
    SyncReturn(RustOpaque::new(RwLock::new(FAContext {
        lang: Lang::Ua,
        context: Vec::new(),
    })))
}

pub struct FAState {
    pub link: Option<String>,
    pub message: String,
    pub next_states: Vec<String>,
}

pub fn get_fs(mlfs: RustOpaque<MultilangFs>, ctx: RWCTX) -> Option<FAState> {
    let ctx = ctx.read().unwrap();
    let fs = ctx
        .context
        .iter()
        .try_fold(mlfs.get(&ctx.lang)?, |fs, key| fs.next_states.get(key))?;
    Some(FAState {
        link: fs.link.clone(),
        message: fs.message.clone(),
        next_states: fs.next_states.keys().map(ToString::to_string).collect(),
    })
}

pub fn transition(ctx: RWCTX, text: String) {
    ctx.write().unwrap().transition(&text);
}
pub fn back(ctx: RWCTX) {
    ctx.write().unwrap().back();
}
pub fn home(ctx: RWCTX) {
    ctx.write().unwrap().home();
}
