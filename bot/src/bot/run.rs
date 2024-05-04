use std::sync::Arc;

use crate::bot::dialogue::prelude::*;
use crate::bot::report_error::FAErrorHandler;
use crate::bot::{FirstAidStorage, State};
use anyhow::{Ok, Result};
use teloxide::dispatching::dialogue::serializer::Bincode;
use teloxide::dispatching::dialogue::RedisStorage;
use teloxide::dptree::case;
use teloxide::prelude::*;
use teloxide::{adaptors::throttle::Limits, types::ParseMode, utils::command::BotCommands};

pub async fn run_bot(storage: Arc<RedisStorage<Bincode>>) -> Result<()> {
    log::info!("Starting dialogue_bot...");

    let bot = Bot::from_env()
        .throttle(Limits::default())
        .parse_mode(ParseMode::MarkdownV2);

    bot.set_my_commands(FACommands::bot_commands()).await?;

    let handler = Update::filter_message()
        .branch(get_commands_branch())
        .branch(get_maintainer_commands_branch())
        .enter_dialogue::<Message, FirstAidStorage, State>()
        .branch(case![State::Start].endpoint(start_endpoint))
        .branch(case![State::Dialogue { lang, context }].endpoint(transition_endpoint))
        .branch(case![State::Broadcast { message }].endpoint(broadcast_endpoint));

    Dispatcher::builder(bot.clone(), handler)
        .dependencies(dptree::deps![storage])
        .error_handler(FAErrorHandler::new(bot))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}
