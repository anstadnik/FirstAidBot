mod dialogue;
mod error_handler;
mod helpers;
mod keyboard;

mod prelude {
    use crate::bot::State;
    use teloxide::adaptors::{DefaultParseMode, Throttle};
    use teloxide::dispatching::dialogue::{serializer::Bincode, RedisStorage};

    pub use super::keyboard::make_keyboard_from_state;
    pub use crate::bot::helpers::send_plain_string;
    pub use crate::model::prelude::*;
    pub use std::sync::Arc;
    pub use teloxide::prelude::*;

    pub type FirstAidDialogue = Dialogue<State, RedisStorage<Bincode>>;
    pub type FirstAirBot = AutoSend<DefaultParseMode<Throttle<Bot>>>;
    pub type FirstAidStorage = RedisStorage<Bincode>;
} /* prelude */

use dialogue::{
    get_commands_branch, get_maintainer_commands_branch, handle_dialogue, reset_dialogue,
    FACommands, State,
};
use error_handler::FirstAidErrorHandler;
use helpers::connect_to_redis;
use prelude::*;
use teloxide::adaptors::throttle::Limits;
use teloxide::types::ParseMode;
use teloxide::utils::command::BotCommands;

pub async fn run_bot(data: Data) {
    log::info!("Starting dialogue_bot...");

    let bot = Bot::from_env()
        .throttle(Limits::default())
        .parse_mode(ParseMode::MarkdownV2)
        .auto_send();

    bot.set_my_commands(FACommands::bot_commands())
        .await
        .unwrap();

    let (redis_con, storage) = connect_to_redis().await;

    let handler = Update::filter_message()
        .branch(get_commands_branch())
        .branch(get_maintainer_commands_branch())
        .enter_dialogue::<Message, FirstAidStorage, State>()
        .branch(dptree::case![State::Start { lang }].endpoint(reset_dialogue))
        .branch(dptree::case![State::Dialogue { lang, context }].endpoint(handle_dialogue));

    Dispatcher::builder(bot.clone(), handler)
        .dependencies(dptree::deps![Arc::new(data), redis_con, storage])
        .error_handler(FirstAidErrorHandler::new(bot))
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}
