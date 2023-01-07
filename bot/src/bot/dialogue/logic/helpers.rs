use super::keyboard::make_keyboard_from_state;
use crate::{bot::prelude::*, MAINTAINER_USERNAMES};
use teloxide::types::ParseMode::Html;

pub async fn send_state(
    bot: &FABot,
    msg: &Message,
    state: &FS,
    lang: Lang,
    context: &[String],
) -> anyhow::Result<()> {
    let id = msg.chat.id;
    if let Some(link) = &state.link {
        let link = format!("<a href='{link}'>&#8288;</a>");
        bot.send_message(id, link).parse_mode(Html).await?;
    }

    let keyboard = make_keyboard_from_state(state, lang, context.len(), is_admin(msg));
    bot.send_message(id, &state.message)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub fn is_admin(msg: &Message) -> bool {
    if let Some(username) = msg.from().and_then(|user| user.username.as_ref()) {
        return MAINTAINER_USERNAMES.contains(&username.as_str());
    }
    false
}
