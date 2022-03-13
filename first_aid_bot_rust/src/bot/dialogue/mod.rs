mod state;

use self::state::move_to_state;
pub use self::state::State;
use super::{helpers::ExtraKeys, Data};
use crate::bot::helpers::{
    get_state, make_keyboard, send_message, GO_BACK_TEXT, GO_TO_BEGINNING_TEXT,
};
use crate::{model::prelude::*, LANGS, REDIS_KEY};
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
    (lang,): (String,),
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
    send_message(&bot, &msg, &data.get().await[&lang], ExtraKeys::empty()).await?;
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
    (lang, mut context): (String, Vec<String>),
) -> anyhow::Result<()> {
    let state = &data.get().await[&lang];
    let FiniteStateOptions { ordered_keys, .. } = get_state(state, &context)
        .await
        .options
        .as_ref()
        .ok_or_else(|| {
            log::error!("There is no options but we're expecting an input: {context:#?}");
            anyhow!(
                "Сталась помилка, будь ласка, повідомте про це у https://t.me/+SvnzzsxStydmNGI6"
            )
        })?;
    match msg.text() {
        Some(GO_TO_BEGINNING_TEXT) => {
            reset_dialogue(bot, msg, data, redis_con, dialogue, (lang,)).await?;
        }
        Some(GO_BACK_TEXT) => {
            context.pop();
            move_to_state(bot, msg, dialogue, data, redis_con, context, lang).await?;
        }
        Some(text) if context.is_empty() && LANGS.iter().any(|lang| lang.text == text) => {
            let lang = LANGS.iter().find(|lang| lang.text == text).unwrap().name;
            reset_dialogue(bot, msg, data, redis_con, dialogue, (lang.to_string(),)).await?;
        }
        Some(text) if ordered_keys.contains(&text.to_string()) => {
            context.push(text.to_string());
            move_to_state(bot, msg, dialogue, data, redis_con, context, lang).await?;
        }
        _ => {
            let keyboard = make_keyboard(ordered_keys, ExtraKeys::new(&context)).await;
            bot.send_message(msg.chat.id, "Використайте кнопки")
                .reply_markup(keyboard)
                .await?;
        }
    }

    Ok(())
}
