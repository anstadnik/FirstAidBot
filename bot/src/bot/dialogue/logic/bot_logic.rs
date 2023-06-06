use first_aid_bot_core::prelude::*;

use super::log_to_redis::log_to_redis;
use super::process_broadcast;
use super::{helpers::send_state, is_admin};
use crate::bot::FABot;
use crate::bot::FADialogue;
use anyhow::{anyhow, Context};
use redis::aio::MultiplexedConnection;
use teloxide::prelude::*;

pub async fn move_to_state(
    bot: &FABot,
    msg: &Message,
    dialogue: &FADialogue,
    fs: &Fs,
    mut ctx: FAContext,
    conn: &mut MultiplexedConnection,
) -> anyhow::Result<()> {
    let map_err = || format!("Error while moving into {ctx}");
    if let Err(err) = log_to_redis(msg, &ctx, conn).await {
        log::error!("Cannot log to redis: {err:?}");
    };
    send_state(bot, msg, &ctx, fs).await.with_context(map_err)?;
    if fs.next_states.is_empty() {
        ctx.home();
        if let Err(err) = log_to_redis(msg, &ctx, conn).await {
            log::error!("Cannot log to redis: {err:?}");
        };
        send_state(bot, msg, &ctx, fs).await?;
    }
    dialogue
        .update(crate::bot::state::State::Dialogue {
            lang: ctx.lang.to_string(),
            context: ctx.context.to_vec(),
        })
        .await?;
    Ok(())
}

pub async fn state_transition(
    bot: &FABot,
    msg: &Message,
    dialogue: &FADialogue,
    mut ctx: FAContext,
    data: &'static Data,
    conn: &mut MultiplexedConnection,
) -> anyhow::Result<()> {
    if let Err(err) = log_to_redis(msg, &ctx, conn).await {
        log::error!("Cannot log to redis: {err:?}");
    };

    let fs = data.get().await?.get_state(&ctx)?;
    match msg.text() {
        Some(text) if text == ctx.lang.details().button_home => ctx.home(),
        Some(text) if text == ctx.lang.details().button_back => ctx.back(),
        Some(text) if fs.next_states.contains_key(text) => ctx.transition(text),
        Some(text) if ctx.is_empty() && Lang::iter().any(|l| l.details().button_lang == text) => {
            ctx.lang = Lang::iter()
                .find(|lang| lang.details().button_lang == text)
                .ok_or_else(|| anyhow!("Wrong language WTF?"))?;
        }
        Some(text) if text == ctx.lang.details().broadcast && is_admin(msg) => {
            dialogue
                .update(crate::bot::state::State::Broadcast { message: None })
                .await?;
            return process_broadcast(bot, msg, dialogue, None, conn).await;
        }
        _ => {
            bot.send_message(msg.chat.id, ctx.lang.details().use_buttons_text)
                .await?;
            return Ok(());
        }
    }

    let new_fs = data.get().await?.get_state(&ctx)?;
    move_to_state(bot, msg, dialogue, &new_fs, ctx, conn).await
}
