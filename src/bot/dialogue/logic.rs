use super::keyboard::make_keyboard_from_state;
use crate::bot::prelude::*;
use anyhow::{anyhow, Context};
use async_recursion::async_recursion;
use redis::{aio::MultiplexedConnection, AsyncCommands};
use std::time::{SystemTime, UNIX_EPOCH};
use teloxide::{
    types::ParseMode,
    utils::markdown::{escape, escape_code},
};

#[async_recursion]
pub async fn move_to_state(
    bot: &FABot,
    msg: &Message,
    dialogue: &FADialogue,
    data: &Arc<Data>,
    context: Vec<String>,
    lang: Lang,
    redis_con: MultiplexedConnection,
) -> anyhow::Result<()> {
    let state = &data.get(lang, &context).await?;
    log_to_redis(msg, &lang, &context, redis_con.clone()).await;
    send_state(bot, msg.chat.id, state, lang, &context).await?;
    if state.next_states.is_empty() {
        return move_to_state(bot, msg, dialogue, data, Vec::new(), lang, redis_con).await;
    }
    let lang = lang.name();
    dialogue.update(State::Dialogue { lang, context }).await?;
    Ok(())
}

pub async fn state_transition(
    bot: &FABot,
    msg: &Message,
    dialogue: &FADialogue,
    data: &Arc<Data>,
    mut context: Vec<String>,
    lang: Lang,
    redis_con: MultiplexedConnection,
) -> anyhow::Result<()> {
    let state = &match data.get(lang, &context).await {
        Ok(it) => it,
        Err(_) => {
            bot.send_message(msg.chat.id, escape_code(lang.details().error_due_to_update))
                .await?;
            return move_to_state(bot, msg, dialogue, data, Vec::new(), lang, redis_con).await;
        }
    };
    log_to_redis(msg, &lang, &context, redis_con.clone()).await;
    match msg.text() {
        Some(text) if text == lang.details().button_home => {
            move_to_state(bot, msg, dialogue, data, Vec::new(), lang, redis_con).await?;
        }
        Some(text) if text == lang.details().button_back => {
            context.pop();
            move_to_state(bot, msg, dialogue, data, context, lang, redis_con).await?;
        }
        Some(text)
            if context.is_empty()
                && Lang::iter().any(|lang| lang.details().button_lang_name == text) =>
        {
            let lang = Lang::iter()
                .find(|lang| lang.details().button_lang_name == text)
                .ok_or_else(|| anyhow!("Wrong language WTF?"))?;
            move_to_state(bot, msg, dialogue, data, Vec::new(), lang, redis_con).await?;
        }
        Some(text) if state.next_states.contains_key(&text.to_string()) => {
            context.push(text.to_string());
            move_to_state(bot, msg, dialogue, data, context.clone(), lang, redis_con)
                .await
                .with_context(|| format!("Error while moving into context {context:?}"))?;
        }
        _ => {
            bot.send_message(msg.chat.id, escape(lang.details().use_buttons_text))
                .await?;
            move_to_state(bot, msg, dialogue, data, context, lang, redis_con).await?;
        }
    }
    anyhow::Ok(())
}

pub async fn send_state(
    bot: &FABot,
    id: ChatId,
    state: &FS,
    lang: Lang,
    context: &[String],
) -> anyhow::Result<()> {
    if let Some(link) = &state.link {
        bot.send_message(id, format!("<a href='{link}'>&#8288;</a>"))
            .parse_mode(ParseMode::Html)
            .await?;
    }

    let keyboard = make_keyboard_from_state(state, lang, context);
    bot.send_message(id, &state.message)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn log_to_redis(
    msg: &Message,
    lang: &Lang,
    context: &[String],
    redis_con: MultiplexedConnection,
) {
    let mut redis_con = redis_con.clone();
    if let Some(user) = msg.from() {
        let user_id = user.id.0.to_string();

        // TODO: Remove it when we'll have dashboards <21-06-22, astadnik> //
        if context.is_empty()
            && redis_con
                .sadd::<_, _, ()>("all_users", &user_id)
                .await
                .is_err()
        {
            log::error!("Error writing a user to the redis db.");
        }

        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let key = "user_".to_string() + &user_id;
        let context = context.join("->");
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
