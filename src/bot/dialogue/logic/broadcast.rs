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
    if !BROADCAST_ENABLED {
        let _ = bot
            .send_message(msg.chat.id, "Alya don't play with it")
            .await;
        dialogue
            .update(State::Start {
                lang: Lang::default().to_string(),
            })
            .await?;
        return Ok(());
    }
    let text = msg
        .text()
        .ok_or_else(|| anyhow!("No text unfortunately, WTF"))?;
    match message {
        None if text == lang.details().broadcast => {
            let keyboard = make_keyboard(&Vec::new(), lang, 42, true);
            bot.send_message(msg.chat.id, "Send your message")
                .reply_markup(keyboard)
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Your message is:").await?;
            let vec = vec![lang.details().confirm.to_string()];
            let keyboard = make_keyboard(&vec, lang, 42, true);
            bot.send_message(msg.chat.id, text)
                .reply_markup(keyboard)
                .await?;

            let lang = lang.to_string();
            let message = Some(text.to_string());
            dialogue.update(State::Broadcast { lang, message }).await?;
        }
        Some(message) => {
            if text == lang.details().confirm {
                bot.send_message(msg.chat.id, lang.details().broadcasting)
                    .await?;

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
                bot.send_message(msg.chat.id, "You didn't confirm, bye")
                    .await?;
                let lang = lang.to_string();
                dialogue.update(State::Start { lang }).await?;
            }
            let keyboard = make_keyboard(&Vec::new(), lang, 42, true);
            bot.send_message(msg.chat.id, "Send your message")
                .reply_markup(keyboard)
                .await?;
        }
    }
    Ok(())
}
