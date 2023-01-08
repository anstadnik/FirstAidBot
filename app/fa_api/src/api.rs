pub use std::sync::RwLock;

use anyhow::Result;
pub use first_aid_bot_core::prelude::{Data, Lang, State};
use flutter_rust_bridge::RustOpaque;
use futures::executor::block_on;

type DataApi = RustOpaque<RwLock<Data>>;

pub fn dynamic() -> DataApi {
    RustOpaque::new(RwLock::new(Data::dynamic()))
}
pub fn cached() -> DataApi {
    RustOpaque::new(RwLock::new(block_on(Data::cached())))
}

type StateApi = RustOpaque<RwLock<State>>;

pub fn get_state(
    data: DataApi,
    ctx: Vec<String>,
    lang: String,
) -> Result<RustOpaque<RwLock<State>>> {
    let lang = lang.as_str().try_into()?;
    block_on(data.read().unwrap().get(&ctx, lang)).map(|v| RustOpaque::new(RwLock::new(v)))
}

pub fn move_to_state(state: StateApi, text: String, data: DataApi) -> Result<()> {
    let data = data.read().unwrap();
    block_on(state.write().unwrap().move_to_state(&text, &data))
}
pub fn back(state: StateApi) {
    state.write().unwrap().back()
}
pub fn home(state: StateApi) {
    state.write().unwrap().home()
}
pub fn depth(state: StateApi) -> usize {
    state.read().unwrap().depth()
}
pub fn is_empty(state: StateApi) -> bool {
    state.read().unwrap().is_empty()
}
pub fn context(state: StateApi) -> Vec<String> {
    state.read().unwrap().context().to_vec()
}

pub fn get_link(state: StateApi) -> Option<String> {
    state.read().unwrap().link.clone()
}
pub fn get_message(state: StateApi) -> String {
    state.read().unwrap().message.clone()
}
pub fn get_button_texts(state: StateApi) -> Vec<String> {
    state.read().unwrap().button_texts.clone()
}
