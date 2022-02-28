mod bot;
mod model;

use std::sync::Arc;

pub use model::{FiniteState, FiniteStateOptions, FiniteStateMsg};
pub use model::get_data;
use teloxide::{dispatching2::dialogue::InMemStorage, prelude2::*};

use crate::bot::State;

pub async fn run_bot(data: Arc<FiniteState>) {
    teloxide::enable_logging!();
    log::info!("Starting dialogue_bot...");

    let bot = Bot::from_env().auto_send();

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .dispatch_by::<State>(),
    )
    .dependencies(dptree::deps![data, InMemStorage::<State>::new()])
    .build()
    .setup_ctrlc_handler()
    .dispatch()
    .await;
}
