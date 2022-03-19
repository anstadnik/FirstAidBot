mod state;

use self::state::move_to_state;
pub use self::state::State;
use super::{helpers::ExtraKeys, Data};
use crate::bot::helpers::{
    get_state, make_keyboard, send_message, GO_BACK_TEXT, GO_TO_BEGINNING_TEXT,
};
use crate::lang::Lang;
use crate::{model::prelude::*, REDIS_KEY};
use anyhow::anyhow;
use redis::{aio::MultiplexedConnection, AsyncCommands};
use std::sync::Arc;
use teloxide::dispatching2::dialogue::{serializer::Bincode, RedisStorage};
use teloxide::{adaptors::DefaultParseMode, prelude2::*};

pub type FirstAidDialogue = Dialogue<State, RedisStorage<Bincode>>;

pub async fn reset_dialogue(
    bot: AutoSend<DefaultParseMode<Bot>>,
    msg: Message,
    data: Arc<Data>,
    mut redis_con: MultiplexedConnection,
    dialogue: FirstAidDialogue,
    (lang,): (Lang,),
) -> anyhow::Result<()> {
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
    dialogue.update(State::Dialogue { lang, context }).await?;
    Ok(())
}

pub async fn handle_dialogue(
    bot: AutoSend<DefaultParseMode<Bot>>,
    msg: Message,
    dialogue: FirstAidDialogue,
    data: Arc<Data>,
    redis_con: MultiplexedConnection,
    (lang, mut context): (Lang, Vec<String>),
) -> anyhow::Result<()> {
    let state = &data.get().await?[&lang];
    let FiniteStateOptions { ordered_keys, .. } =
        get_state(state, &context).options.as_ref().ok_or_else(|| {
            log::error!("There are no options but we're expecting an input: {context:#?}");
            anyhow!(lang.details().error)
        })?;
    match msg.text() {
        Some(GO_TO_BEGINNING_TEXT) => {
            reset_dialogue(bot, msg, data, redis_con, dialogue, (lang,)).await?;
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
            reset_dialogue(bot, msg, data, redis_con, dialogue, (lang,)).await?;
        }
        Some(text) if ordered_keys.contains(&text.to_string()) => {
            context.push(text.to_string());
            move_to_state(bot, msg, dialogue, data, redis_con, context, lang).await?;
        }
        _ => {
            let keyboard = make_keyboard(ordered_keys, ExtraKeys::new(&context, Some(lang)));
            bot.send_message(msg.chat.id, lang.details().use_buttons_text)
                .reply_markup(keyboard)
                .await?;
        }
    }

    Ok(())
}
