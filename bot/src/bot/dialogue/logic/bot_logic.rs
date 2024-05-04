use super::keyboard::make_keyboard;
use super::{log_to_redis::log_to_redis, process_broadcast};
use crate::bot::{FABot, FADialogue, State};
use crate::{DataGetState, DATA, MAINTAINER_USERNAMES};
use anyhow::{anyhow, Context, Result};
use first_aid_bot_core::prelude::*;
use teloxide::prelude::*;
use teloxide::types::ParseMode::Html;

pub async fn send_state(bot: &FABot, msg: &Message, ctx: &FAContext, fs: &Fs) -> Result<()> {
    let _ = log_to_redis(msg, ctx).await;
    let id = msg.chat.id;
    if let Some(link) = &fs.link {
        let link = format!("<a href='{link}'>&#8288;</a>");
        bot.send_message(id, link).parse_mode(Html).await?;
    }

    let keys = fs.next_states.keys().map(AsRef::as_ref).collect::<Vec<_>>();
    let kbd = make_keyboard(&keys, ctx.lang, ctx.depth(), is_admin(msg));
    bot.send_message(id, &fs.message).reply_markup(kbd).await?;
    Ok(())
}

pub fn is_admin(msg: &Message) -> bool {
    msg.from()
        .and_then(|user| user.username.as_ref())
        .is_some_and(|username| MAINTAINER_USERNAMES.contains(&username.as_str()))
}

pub async fn move_to_state(
    bot: &FABot,
    msg: &Message,
    dialogue: &FADialogue,
    fs: &Fs,
    mut ctx: FAContext,
) -> anyhow::Result<()> {
    send_state(bot, msg, &ctx, fs)
        .await
        .context(format!("Error while sending {ctx}"))?;
    if fs.next_states.is_empty() {
        ctx.home();
        send_state(bot, msg, &ctx, fs)
            .await
            .context(format!("Error while sending {ctx}"))?;
    }
    let lang = ctx.lang.to_string();
    let context = ctx.context;
    dialogue.update(State::Dialogue { lang, context }).await?;
    Ok(())
}

pub async fn transition_logic(
    bot: &FABot,
    msg: &Message,
    dialogue: &FADialogue,
    mut ctx: FAContext,
) -> anyhow::Result<()> {
    let fs = DATA.get_state(&ctx).await?;
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
            return process_broadcast(bot, msg, dialogue, None).await;
        }
        _ => {
            bot.send_message(msg.chat.id, ctx.lang.details().use_buttons_text)
                .await?;
            return Ok(());
        }
    }

    let new_fs = DATA.get_state(&ctx).await?;
    move_to_state(bot, msg, dialogue, &new_fs, ctx).await
}
