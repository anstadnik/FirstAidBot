use super::keyboard::make_keyboard;
use crate::bot::prelude::*;
use anyhow::{anyhow, Result};
use futures::{stream, StreamExt};
use redis::{aio::MultiplexedConnection, AsyncCommands};

// TODO: Think what to do about multilang when we'll have it
pub async fn process_broadcast(
    bot: &FABot,
    msg: &Message,
    dialogue: &FADialogue,
    message: Option<String>,
    lang: Lang,
    conn: &mut MultiplexedConnection,
) -> Result<()> {
    let text = msg
        .text()
        .ok_or_else(|| anyhow!("No text unfortunately, WTF"))?;
    match message {
        None if text == lang.details().broadcast => {
            let keyboard = make_keyboard(&Vec::new(), lang, 0, true);
            bot.send_message(msg.chat.id, "Send your message")
                .reply_markup(keyboard)
                .await?;
        }
        None => {
            dialogue
                .update(State::Broadcast {
                    lang: lang.to_string(),
                    message: Some(text.to_string()),
                })
                .await?;
            bot.send_message(msg.chat.id, "Your message is:").await?;
            let vec = vec![lang.details().confirm.to_string()];
            let keyboard = make_keyboard(&vec, lang, 0, true);
            bot.send_message(msg.chat.id, text)
                .reply_markup(keyboard)
                .await?;
        }
        Some(message) => {
            if text == lang.details().confirm {
                bot.send_message(msg.chat.id, lang.details().broadcasting)
                    .await?;

                async fn send_to_user(bot: &FABot, user_id: &str, message: &str) -> Result<()> {
                    bot.send_message(UserId(user_id.parse::<u64>()?), message)
                        .await?;
                    Ok(())
                }

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
            let keyboard = make_keyboard(&Vec::new(), lang, 0, true);
            bot.send_message(msg.chat.id, "Send your message")
                .reply_markup(keyboard)
                .await?;
        }
    }
    Ok(())
}
