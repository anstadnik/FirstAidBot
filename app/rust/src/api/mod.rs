#![allow(dead_code)]
mod api;
mod bridge_generated;
pub use first_aid_bot_core::prelude::*;
use flutter_rust_bridge::{RustOpaque, SyncReturn};
pub use std::sync::RwLock;

#[tokio::main]
pub async fn get_data() -> Option<RustOpaque<MultilangFs>> {
    get_data_from_web().await.map(RustOpaque::new).ok()
    // get_data_from_file("/Users/astadnik/Ukraine/FirstAidBot/table.csv").map(RustOpaque::new)
}

type Rwctx = RustOpaque<RwLock<FAContext>>;
pub fn get_context() -> SyncReturn<Rwctx> {
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

pub fn get_fs(mlfs: RustOpaque<MultilangFs>, ctx: Rwctx) -> Option<FAState> {
    let ctx = ctx.read().unwrap();
    let fs = ctx
        .context
        .iter()
        .try_fold(mlfs.get(&ctx.lang)?, |fs: &Fs, key| fs.next_states.get(key))?;
    Some(FAState {
        link: fs.link.clone(),
        message: fs.message.clone(),
        next_states: fs.next_states.keys().map(ToString::to_string).collect(),
    })
}

pub fn transition(ctx: Rwctx, text: String) {
    ctx.write().unwrap().transition(text);
}
pub fn back(ctx: Rwctx) {
    ctx.write().unwrap().back();
}
pub fn home(ctx: Rwctx) {
    ctx.write().unwrap().home();
}

pub fn get_path(ctx: Rwctx) -> String {
    ctx.read().unwrap().to_string()
}
