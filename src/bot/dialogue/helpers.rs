use super::prelude::*;
use redis::AsyncCommands;
use std::time::{SystemTime, UNIX_EPOCH};
use teloxide::{prelude::*, types::ParseMode};

pub async fn log_to_redis(args: &FAMsgArgs<'_>, lang: &Lang, context: &[String]) {
    let FAMsgArgs { msg, redis_con, .. }: &FAMsgArgs = args;
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