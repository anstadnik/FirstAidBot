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

    let is_admin = msg.from().is_some_and(|user| {
        user.username
            .is_some_and(|username| MAINTAINER_USERNAMES.contains(&username.as_str()))
    });

    let keyboard = make_keyboard_from_state(state, lang, context, is_admin);
    bot.send_message(msg.chat.id, &state.message)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}
