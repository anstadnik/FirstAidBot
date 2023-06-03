use first_aid_bot_core::prelude::*;
use std::sync::Arc;

use super::log_to_redis::log_to_redis;
use super::process_broadcast;
use super::{helpers::send_state, is_admin};
use crate::bot::FABot;
use crate::bot::FADialogue;
use anyhow::{anyhow, Context as AnyhowContext};
use redis::aio::MultiplexedConnection;
use teloxide::prelude::*;

pub async fn move_to_state(
    bot: &FABot,
    msg: &Message,
    dialogue: &FADialogue,
    mut state: State,
    conn: &mut MultiplexedConnection,
) -> anyhow::Result<()> {
    let map_err = || format!("Error while moving into {state}");
    if let Err(err) = log_to_redis(msg, &state, conn).await {
        log::error!("Cannot log to redis: {err:?}");
    };
    send_state(bot, msg, &state).await.with_context(map_err)?;
    if state.button_texts.is_empty() {
        state.home();
        if let Err(err) = log_to_redis(msg, &state, conn).await {
            log::error!("Cannot log to redis: {err:?}");
        };
        send_state(bot, msg, &state).await?;
    }
    dialogue
        .update(crate::bot::state::State::Dialogue {
            lang: state.lang.to_string(),
            context: state.context().to_vec(),
        })
        .await?;
    Ok(())
}

pub async fn state_transition(
    bot: &FABot,
    msg: &Message,
    dialogue: &FADialogue,
    mut state: State,
    data: Arc<Data>,
    conn: &mut MultiplexedConnection,
) -> anyhow::Result<()> {
    if let Err(err) = log_to_redis(msg, &state, conn).await {
        log::error!("Cannot log to redis: {err:?}");
    };

    match msg.text() {
        Some(text) if text == state.lang.details().button_home => state.home(),
        Some(text) if text == state.lang.details().button_back => state.back(),
        Some(text) if state.button_texts.iter().any(|t| t == text) => {
            state.move_to_state(text, &data).await?
        }
        Some(text) if state.is_empty() && Lang::iter().any(|l| l.details().button_lang == text) => {
            state.lang = Lang::iter()
                .find(|lang| lang.details().button_lang == text)
                .ok_or_else(|| anyhow!("Wrong language WTF?"))?;
        }
        Some(text) if text == state.lang.details().broadcast && is_admin(msg) => {
            dialogue
                .update(crate::bot::state::State::Broadcast { message: None })
                .await?;
            return process_broadcast(bot, msg, dialogue, None, conn).await;
        }
        _ => {
            bot.send_message(msg.chat.id, state.lang.details().use_buttons_text)
                .await?;
            return Ok(());
        }
    }
    move_to_state(bot, msg, dialogue, state, conn).await
}
