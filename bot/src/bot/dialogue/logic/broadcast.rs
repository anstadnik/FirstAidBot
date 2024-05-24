use super::keyboard::make_keyboard;
use crate::bot::FABot;
use crate::bot::FADialogue;
use crate::bot::State;
use crate::BROADCAST_ENABLED;
use crate::REDIS_CONN;
use anyhow::{anyhow, Result};
use first_aid_bot_core::prelude::Lang;
use futures::{stream, StreamExt};
use redis::AsyncCommands;
use teloxide::prelude::*;

const MSG_DISABLED: &str = "Аля, ні";
const MSG_CONFIRM: &str = "Підтвердити";
const MSG_BROADCASTING: &str = "Надсилаю\\! З Богом";
const MSG_REQUEST: &str = "Надішли мені повідомлення";

async fn send_to_user(bot: &FABot, user_id: &str, message: &str) -> Result<()> {
    let err = "Cannot parse user_id";
    let user_id = user_id.strip_prefix("user_").ok_or_else(|| anyhow!(err));
    let chat_id = UserId(user_id?.parse::<u64>()?);
    bot.send_message(chat_id, message).await?;
    Ok(())
}

// TODO: Think what to do about multilang when we'll have it
pub async fn process_broadcast(
    bot: &FABot,
    msg: &Message,
    dialogue: &FADialogue,
    message: Option<String>,
) -> Result<()> {
    let text = msg.text().ok_or_else(|| anyhow!("No text unfortunately"))?;
    let id = msg.chat.id;

    if !BROADCAST_ENABLED {
        std::mem::drop(bot.send_message(id, MSG_DISABLED).await);
        dialogue.update(State::Start).await?;
        return Ok(());
    }

    match message {
        None if text == MSG_BROADCASTING => wait_for_message(bot, id).await,
        None => ask_to_confirm(bot, id, text, dialogue).await,
        Some(msg) => broadcast_if_confirmed(text, bot, id, &msg, dialogue).await,
    }
}

async fn broadcast_if_confirmed(
    text: &str,
    bot: &FABot,
    id: ChatId,
    message: &str,
    dialogue: &FADialogue,
) -> Result<()> {
    if text == MSG_CONFIRM {
        bot.send_message(id, MSG_BROADCASTING).await?;

        let mut conn = REDIS_CONN
            .get()
            .ok_or_else(|| anyhow!("No connection"))?
            .clone();
        stream::iter(conn.keys::<_, Vec<String>>("user_*").await?)
            .for_each(|user_id| async move {
                if let Err(err) = send_to_user(bot, &user_id, message).await {
                    // TODO: Should be sync I guess
                    log::error!("Cannot broadcast to {user_id}: {err:?}");
                }
            })
            .await;
    } else {
        bot.send_message(id, "You didn't confirm, bye").await?;
        dialogue.update(State::Start).await?;
    }
    Ok(())
}

async fn ask_to_confirm(bot: &FABot, id: ChatId, text: &str, dialogue: &FADialogue) -> Result<()> {
    bot.send_message(id, "Your message is:").await?;
    let vec = vec![MSG_CONFIRM];
    let keyboard = make_keyboard(&vec, Lang::En, 42, true);
    bot.send_message(id, text).reply_markup(keyboard).await?;
    let message = Some(text.to_string());
    dialogue.update(State::Broadcast { message }).await?;
    Ok(())
}

async fn wait_for_message(bot: &FABot, id: ChatId) -> Result<()> {
    let kbd = make_keyboard(&Vec::new(), Lang::En, 42, true);
    bot.send_message(id, MSG_REQUEST).reply_markup(kbd).await?;
    Ok(())
}
