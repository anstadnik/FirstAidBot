mod dialogue;
mod report_error;
mod run;
mod state;

use teloxide::adaptors::{DefaultParseMode, Throttle};
use teloxide::dispatching::dialogue::{serializer::Bincode, RedisStorage};
use teloxide::prelude::*;

use self::state::State;
pub type FADialogue = Dialogue<State, RedisStorage<Bincode>>;
pub type FABot = DefaultParseMode<Throttle<Bot>>;
pub type FirstAidStorage = RedisStorage<Bincode>;
pub mod prelude {
    pub use super::run::run_bot;
}
