use super::{helpers::send_state, is_admin, log_to_redis::log_to_redis, process_broadcast};
use crate::bot::{FABot, FADialogue, State};
use anyhow::{anyhow, Context};
use first_aid_bot_core::prelude::*;
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
    let err = format!("Error while moving into {ctx}");
    let _ = log_to_redis(msg, &ctx, conn).await;
    send_state(bot, msg, &ctx, fs).await.context(err.clone())?;
    if fs.next_states.is_empty() {
        ctx.home();
        let _ = log_to_redis(msg, &ctx, conn).await;
        send_state(bot, msg, &ctx, fs).await.context(err)?;
    }
    dialogue
        .update(State::Dialogue {
            lang: ctx.lang.to_string(),
            context: ctx.context,
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
    let fs = data.get().await?.get_state(&ctx)?;
    match msg.text() {
        Some(text) if text == ctx.lang.details().button_home => ctx.home(),
        Some(text) if text == ctx.lang.details().button_back => ctx.back(),
        Some(text) if fs.next_states.contains_key(text) => ctx.transition(text.to_string()),
        Some(text) if ctx.is_empty() && Lang::iter().any(|l| l.details().button_lang == text) => {
            ctx.lang = Lang::iter()
                .find(|lang| lang.details().button_lang == text)
                .ok_or(anyhow!("Wrong language WTF?"))?;
        }
        Some(text) if text == ctx.lang.details().broadcast && is_admin(msg) => {
            dialogue.update(State::Broadcast { message: None }).await?;
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
