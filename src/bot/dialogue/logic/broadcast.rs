use super::keyboard::make_keyboard;
use crate::{bot::prelude::*, BROADCAST_ENABLED};
use anyhow::{anyhow, Result};
use futures::{stream, StreamExt};
use redis::{aio::MultiplexedConnection, AsyncCommands};

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
    lang: Lang,
    conn: &mut MultiplexedConnection,
) -> Result<()> {
    let text = msg.text().ok_or_else(|| anyhow!("No text unfortunately"))?;
    let id = msg.chat.id;

    if !BROADCAST_ENABLED {
        let _ = bot.send_message(id, "Alya don't play with it").await;
        let lang = Lang::default().to_string();
        dialogue.update(State::Start { lang }).await?;
        return Ok(());
    }

    match message {
        None if text == lang.details().broadcast => wait_for_message(lang, bot, id).await,
        None => ask_to_confirm(bot, id, lang, text, dialogue).await,
        Some(msg) => broadcast_if_confirmed(text, lang, bot, id, msg, conn, dialogue).await,
    }
}

async fn broadcast_if_confirmed(
    text: &str,
    lang: Lang,
    bot: &FABot,
    id: ChatId,
    message: String,
    conn: &mut MultiplexedConnection,
    dialogue: &FADialogue,
) -> Result<()> {
    if text == lang.details().confirm {
        bot.send_message(id, lang.details().broadcasting).await?;

        let message = &message;
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
        let lang = lang.to_string();
        dialogue.update(State::Start { lang }).await?;
    }
    let keyboard = make_keyboard(&Vec::new(), lang, 42, true);
    bot.send_message(id, "Send your message")
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

async fn ask_to_confirm(
    bot: &FABot,
    id: ChatId,
    lang: Lang,
    text: &str,
    dialogue: &FADialogue,
) -> Result<()> {
    bot.send_message(id, "Your message is:").await?;
    let vec = vec![lang.details().confirm.to_string()];
    let keyboard = make_keyboard(&vec, lang, 42, true);
    bot.send_message(id, text).reply_markup(keyboard).await?;
    let lang = lang.to_string();
    let message = Some(text.to_string());
    dialogue.update(State::Broadcast { lang, message }).await?;
    Ok(())
}

async fn wait_for_message(lang: Lang, bot: &FABot, id: ChatId) -> Result<()> {
    let keyboard = make_keyboard(&Vec::new(), lang, 42, true);
    let text = "Send your message";
    bot.send_message(id, text).reply_markup(keyboard).await?;
    Ok(())
}
