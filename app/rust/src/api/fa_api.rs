use flutter_rust_bridge::frb;
use std::{borrow::Cow, env::set_var};

use anyhow::{bail, Result};
pub use first_aid_bot_core::prelude::*;

#[frb(external)]
impl FAContext {
    pub fn transition(&mut self, _text: String) {}
    pub fn back(&mut self) {}
    pub fn home(&mut self) {}
    pub fn depth(&self) -> usize {}
    pub fn is_empty(&self) -> bool {}
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

pub async fn get_data() -> Result<Data> {
    // TODO: Change to static
    Data::download().await
}

#[frb(sync)]
pub fn get_context() -> FAContext {
    FAContext::default()
}

pub struct Fs_ {
    pub link: Option<String>,
    pub message: String,
    pub next_states: Vec<String>,
}

pub async fn get_state<'a>(data: &'a Data, ctx: &FAContext) -> Result<Fs_> {
    let Cow::Borrowed(Fs {
        link,
        message,
        next_states,
    }) = data.get().await?.get_state(ctx)?
    else {
        bail!("Data is not borrowed!")
    };
    Ok(Fs_ {
        link: link.clone(),
        message: message.clone(),
        next_states: next_states.keys().cloned().collect(),
    })
}
