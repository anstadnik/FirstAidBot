pub use std::sync::RwLock;

use anyhow::Result;
pub use first_aid_bot_core::prelude::{Data, Lang, State};
use flutter_rust_bridge::{RustOpaque, SyncReturn};
use tokio::runtime::Runtime;

pub fn get_dynamic() -> SyncReturn<RustOpaque<RwLock<Data>>> {
    SyncReturn(RustOpaque::new(RwLock::new(Data::dynamic())))
}
pub fn get_cached() -> RustOpaque<RwLock<Data>> {
    let rt = Runtime::new().unwrap();
    RustOpaque::new(RwLock::new(rt.block_on(Data::cached())))
}

type StateApi = RustOpaque<RwLock<State>>;

// pub fn get_state(
//     data: RustOpaque<RwLock<Data>>,
//     ctx: Vec<String>,
//     lang: String,
// ) -> Result<RustOpaque<RwLock<State>>> {
//     let rt = Runtime::new().unwrap();
//     let lang = lang.as_str().try_into()?;
//     rt.block_on(data.read().unwrap().get(&ctx, lang))
//         .map(|v| RustOpaque::new(RwLock::new(v)))
// }

pub fn get_state(
    data: RustOpaque<RwLock<Data>>,
    ctx: Vec<String>,
    lang: String,
) -> SyncReturn<RustOpaque<RwLock<State>>> {
    let rt = Runtime::new().unwrap();
    let lang = lang.as_str().try_into().unwrap();
    SyncReturn(
        rt.block_on(data.read().unwrap().get(&ctx, lang))
            .map(|v| RustOpaque::new(RwLock::new(v))).unwrap(),
    )
}

pub fn move_to_state(state: StateApi, text: String, data: RustOpaque<RwLock<Data>>) -> Result<()> {
    let rt = Runtime::new().unwrap();
    let data = data.read().unwrap();
    rt.block_on(state.write().unwrap().move_to_state(&text, &data))
}
pub fn back(state: StateApi) {
    state.write().unwrap().back()
}
pub fn home(state: StateApi) {
    state.write().unwrap().home()
}
pub fn depth(state: StateApi) -> SyncReturn<usize> {
    SyncReturn(state.read().unwrap().depth())
}
pub fn is_empty(state: StateApi) -> SyncReturn<bool> {
    SyncReturn(state.read().unwrap().is_empty())
}
pub fn context(state: StateApi) -> SyncReturn<Vec<String>> {
    SyncReturn(state.read().unwrap().context().to_vec())
}

pub fn get_link(state: StateApi) -> SyncReturn<Option<String>> {
    SyncReturn(state.read().unwrap().link.clone())
}
pub fn get_message(state: StateApi) -> SyncReturn<String> {
    SyncReturn(state.read().unwrap().message.clone())
}
pub fn get_button_texts(state: StateApi) -> SyncReturn<Vec<String>> {
    SyncReturn(state.read().unwrap().button_texts.clone())
}
