mod dialogue;
mod error_handler;
mod helpers;
mod run;
mod state;

mod prelude {
    use teloxide::adaptors::{DefaultParseMode, Throttle};
    use teloxide::dispatching::dialogue::{serializer::Bincode, RedisStorage};

    pub use crate::bot::helpers::send_plain_string;
    pub use crate::model::prelude::*;
    pub use std::sync::Arc;
    pub use teloxide::prelude::*;
    pub use super::state::State;

    pub type FADialogue = Dialogue<State, RedisStorage<Bincode>>;
    pub type FABot = AutoSend<DefaultParseMode<Throttle<Bot>>>;
    pub type FirstAidStorage = RedisStorage<Bincode>;
} /* prelude */

pub use run::run_bot;
