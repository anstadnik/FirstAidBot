mod commands;
mod dialogue;
mod helpers;

use crate::{
    bot::{
        commands::{commands_handler, FirstAidCommands},
        dialogue::State,
    },
    model::FiniteState,
};
use std::sync::Arc;
use teloxide::{dispatching2::dialogue::InMemStorage, prelude2::*};

pub async fn run_bot(data: Arc<FiniteState>) {
    teloxide::enable_logging!();
    log::info!("Starting dialogue_bot...");

    let bot = Bot::from_env()
        .auto_send();

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<FirstAidCommands>()
                .enter_dialogue::<Message, InMemStorage<State>, State>()
                .endpoint(commands_handler),
        )
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .dispatch_by::<State>();

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![data, InMemStorage::<State>::new()])
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}
