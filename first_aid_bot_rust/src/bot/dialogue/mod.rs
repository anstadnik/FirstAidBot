mod state;

use self::state::move_to_state;
pub use self::state::State;
use super::{helpers::ExtraKeys, Data};
use crate::HELP_CHAT_URL;
use crate::bot::helpers::{
    get_state, make_keyboard, send_error, send_message, GO_BACK_TEXT, GO_TO_BEGINNING_TEXT,
};
use crate::lang::Lang;
use crate::{model::prelude::*, REDIS_KEY};
use anyhow::anyhow;
use redis::{aio::MultiplexedConnection, AsyncCommands};
use teloxide::adaptors::Throttle;
use std::sync::Arc;
use teloxide::dispatching::dialogue::{serializer::Bincode, RedisStorage};
use teloxide::types::ParseMode;
use teloxide::{adaptors::DefaultParseMode, prelude::*};

pub type FirstAidDialogue = Dialogue<State, RedisStorage<Bincode>>;

pub async fn reset_dialogue(
    bot: AutoSend<DefaultParseMode<Throttle<Bot>>>,
    msg: Message,
    data: Arc<Data>,
    mut redis_con: MultiplexedConnection,
    dialogue: FirstAidDialogue,
    lang: String,
) -> anyhow::Result<()> {
    let lang = match lang.as_str().try_into() {
        Ok(lang) => lang,
        Err(_) => {
            log::error!("Unknown language: {lang}");
            let err = "Error, please choose a language again. Ask for help in ".to_string() + HELP_CHAT_URL;
            send_error(&bot, &msg, err.to_string()).await?;
            Lang::default()
        }
    };
    if let Some(user) = msg.from() {
        if redis_con
            .sadd::<&str, String, ()>(REDIS_KEY, user.id.to_string())
            .await
            .is_err()
        {
            log::error!("Error writing a user to the redis db.");
        }
    }
    send_message(
        &bot,
        &msg,
        &data.get().await?[&lang],
        ExtraKeys::empty(lang),
    )
    .await?;
    let context = vec![];
    dialogue.update(State::Dialogue { lang: lang.name(), context }).await?;
    Ok(())
}

pub async fn handle_dialogue(
    bot: AutoSend<DefaultParseMode<Throttle<Bot>>>,
    msg: Message,
    dialogue: FirstAidDialogue,
    data: Arc<Data>,
    redis_con: MultiplexedConnection,
    (lang, mut context): (String, Vec<String>),
) -> anyhow::Result<()> {
    let lang = match lang.as_str().try_into() {
        Ok(lang) => lang,
        Err(_) => {
            log::error!("Unknown language: {lang}");
            let err = "Unknown language, please report it to ".to_string() + HELP_CHAT_URL;
            send_error(&bot, &msg, err.to_string()).await?;
            Lang::default()
        }
    };
    let state = &data.get().await?[&lang];
    let FiniteStateOptions { ordered_keys, .. } =
        get_state(state, &context).options.as_ref().ok_or_else(|| {
            log::error!("There are no options but we're expecting an input: {context:#?}");
            anyhow!(lang.details().error)
        })?;
    match msg.text() {
        Some(GO_TO_BEGINNING_TEXT) => {
            reset_dialogue(bot, msg, data, redis_con, dialogue, lang.name()).await?;
        }
        Some(GO_BACK_TEXT) => {
            context.pop();
            move_to_state(bot, msg, dialogue, data, redis_con, context, lang).await?;
        }
        Some(text)
            if context.is_empty()
                && Lang::iter().any(|lang| lang.details().button_text == text) =>
        {
            let lang = Lang::iter()
                .find(|lang| lang.details().button_text == text)
                .unwrap();
            reset_dialogue(bot, msg, data, redis_con, dialogue, lang.name()).await?;
        }
        Some(text) if ordered_keys.contains(&text.to_string()) => {
            context.push(text.to_string());
            move_to_state(bot, msg, dialogue, data, redis_con, context, lang).await?;
        }
        _ => {
            let keyboard = make_keyboard(ordered_keys, ExtraKeys::new(&context, Some(lang)));
            #[allow(deprecated)]
            bot.send_message(msg.chat.id, lang.details().use_buttons_text)
                .parse_mode(ParseMode::Markdown)
                .reply_markup(keyboard)
                .await?;
        }
    }

    Ok(())
}
