use std::sync::Arc;

use anyhow::anyhow;
use redis::{aio::MultiplexedConnection, AsyncCommands};
use teloxide::{
    adaptors::DefaultParseMode,
    dispatching2::dialogue::{serializer::Bincode, RedisStorage},
    macros::DialogueState,
    prelude2::*,
};

use crate::{
    bot::helpers::{get_state, make_keyboard, send_message, GO_BACK_TEXT, GO_TO_BEGINNING_TEXT},
    model::{FiniteState, FiniteStateOptions},
    REDIS_KEY,
};

pub type FirstAidDialogue = Dialogue<State, RedisStorage<Bincode>>;

#[derive(DialogueState, Clone, serde::Serialize, serde::Deserialize)]
#[handler_out(anyhow::Result<()>)]
pub enum State {
    #[handler(reset_dialogue)]
    Start,

    #[handler(handle_dialogue)]
    Dialogue { context: Vec<String> },
}

impl Default for State {
    fn default() -> Self {
        Self::Start
    }
}

pub async fn reset_dialogue(
    bot: AutoSend<DefaultParseMode<Bot>>,
    msg: Message,
    data: Arc<FiniteState>,
    mut redis_con: MultiplexedConnection,
    dialogue: FirstAidDialogue,
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
    send_message(&bot, &msg, &data).await?;
    dialogue.update(State::Dialogue { context: vec![] }).await?;
    Ok(())
}

async fn move_to_state(
    bot: AutoSend<DefaultParseMode<Bot>>,
    msg: Message,
    dialogue: FirstAidDialogue,
    data: Arc<FiniteState>,
    redis_con: MultiplexedConnection,
    context: Vec<String>,
) -> anyhow::Result<()> {
    let state = get_state(data.as_ref(), &context).await;
    send_message(&bot, &msg, state).await?;
    if state.options.is_none() {
        return reset_dialogue(bot, msg, data, redis_con, dialogue).await;
    }
    dialogue.update(State::Dialogue { context }).await?;
    Ok(())
}

async fn handle_dialogue(
    bot: AutoSend<DefaultParseMode<Bot>>,
    msg: Message,
    dialogue: FirstAidDialogue,
    data: Arc<FiniteState>,
    redis_con: MultiplexedConnection,
    (mut context,): (Vec<String>,),
) -> anyhow::Result<()> {
    let FiniteStateOptions { ordered_keys, .. } = get_state(data.as_ref(), &context)
        .await
        .options
        .as_ref()
        .ok_or_else(|| {
            log::error!("There is no options but we're expecting an input: {context:#?}");
            anyhow!("Сталась помилка, будь ласка, повідомте про це у https://t.me/+SvnzzsxStydmNGI6")
        })?;
    match msg.text() {
        Some(GO_TO_BEGINNING_TEXT) => {
            reset_dialogue(bot, msg, data, redis_con, dialogue).await?;
        }
        Some(GO_BACK_TEXT) => {
            context.pop();
            move_to_state(bot, msg, dialogue, data, redis_con, context).await?;
        }
        Some(text) if ordered_keys.contains(&text.to_string()) => {
            context.push(text.to_string());
            move_to_state(bot, msg, dialogue, data, redis_con, context).await?;
        }
        _ => {
            let keyboard = make_keyboard(ordered_keys).await;
            bot.send_message(msg.chat.id, "Використайте кнопки")
                .reply_markup(keyboard)
                .await?;
        }
    }

    Ok(())
}
