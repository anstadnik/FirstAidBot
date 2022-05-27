use super::prelude::*;
use anyhow::bail;
use redis::{aio::MultiplexedConnection, AsyncCommands};
use std::time::{SystemTime, UNIX_EPOCH};
use teloxide::types::{Message, ReplyMarkup};
use teloxide::{prelude::*, types::ParseMode};

pub async fn get_lang_or_warn_and_default(
    bot: &FirstAidBot,
    msg: &Message,
    lang: String,
) -> anyhow::Result<Lang> {
    Ok(match lang.as_str().try_into() {
        Ok(lang) => lang,
        Err(err) => {
            send_message(bot, msg.chat.id, err).await?;
            Lang::default()
        }
    })
}

pub async fn log_to_redis(
    msg: &Message,
    redis_con: &mut MultiplexedConnection,
    lang: &Lang,
    context: Option<&Vec<String>>,
) {
    if let Some(user) = msg.from() {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let key = "user_".to_string() + &user.id.0.to_string();
        let context = context
            .map(|context| context.join("->"))
            .unwrap_or_default();
        let value = format!("{context}; {lang}");
        if redis_con
            .hset::<_, _, _, ()>(key, time, value)
            .await
            .is_err()
        {
            log::error!("Error writing a user to the redis db.");
        }
    };
}

pub async fn send_state(
    bot: &FirstAidBot,
    msg: &Message,
    state: &FiniteState,
    lang: Lang,
    keyboard: ReplyMarkup,
) -> anyhow::Result<()> {
    if let Some(link) = &state.link {
        bot.send_message(msg.chat.id, format!("<a href='{link}'>&#8288;</a>"))
            .parse_mode(ParseMode::Html)
            .await?;
    }

    let rez = bot
        .send_message(msg.chat.id, &state.message)
        .reply_markup(keyboard)
        .await;

    if let Err(err) = rez {
        send_message(bot, msg.chat.id, lang.details().error.to_string()).await?;
        send_message(bot, msg.chat.id, format!("{err:#?}")).await?;
        send_message(bot, msg.chat.id, state.message.to_owned()).await?;
        bail!(err);
    }
    Ok(())
}
