mod dialogue;
mod report_error;
mod run;

use teloxide::adaptors::{DefaultParseMode, Throttle};
use teloxide::dispatching::dialogue::{serializer::Bincode, RedisStorage};
use teloxide::prelude::*;

pub type FADialogue = Dialogue<State, RedisStorage<Bincode>>;
pub type FABot = DefaultParseMode<Throttle<Bot>>;
pub type FirstAidStorage = RedisStorage<Bincode>;
pub mod prelude {
    pub use super::run::run_bot;
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
pub enum State {
    #[default]
    Start,
    Dialogue {
        lang: String,
        context: Vec<String>,
    },
    Broadcast {
        message: Option<String>,
    },
}
