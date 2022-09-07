mod dialogue;
mod error_handler;
mod run;
mod state;

mod prelude {
    use teloxide::adaptors::{DefaultParseMode, Throttle};
    use teloxide::dispatching::dialogue::{serializer::Bincode, RedisStorage};

    pub use crate::model::prelude::*;
    pub use std::sync::Arc;
    pub use teloxide::prelude::*;
    pub use super::state::State;

    pub type FADialogue = Dialogue<State, RedisStorage<Bincode>>;
    pub type FABot = AutoSend<DefaultParseMode<Throttle<Bot>>>;
    pub type FirstAidStorage = RedisStorage<Bincode>;
}

pub use run::run_bot;
