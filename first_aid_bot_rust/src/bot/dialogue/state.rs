use super::prelude::*;
// use crate::bot::keyboard::make_keyboard_from_state;
// use redis::aio::MultiplexedConnection;
// use std::sync::Arc;
// use teloxide::types::Message;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum State {
    Start { lang: String },
    Dialogue { lang: String, context: Vec<String> },
}

impl Default for State {
    fn default() -> Self {
        Self::Start {
            lang: Lang::default().name(),
        }
    }
}
