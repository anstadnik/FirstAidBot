use super::keyboard::make_keyboard_from_state;
use crate::{bot::prelude::*, MAINTAINER_USERNAMES};
use teloxide::types::ParseMode;

pub async fn send_state(
    bot: &FABot,
    msg: &Message,
    state: &FS,
    lang: Lang,
    context: &[String],
) -> anyhow::Result<()> {
    if let Some(link) = &state.link {
        bot.send_message(msg.chat.id, format!("<a href='{link}'>&#8288;</a>"))
            .parse_mode(ParseMode::Html)
            .await?;
    }

    let keyboard = make_keyboard_from_state(state, lang, context, is_admin(msg));
    bot.send_message(msg.chat.id, &state.message)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub fn is_admin(msg: &Message) -> bool {
    if let Some(user) = msg.from() {
        if let Some(username) = &user.username {
            return MAINTAINER_USERNAMES.contains(&username.as_str());
        }
    }
    false
}
