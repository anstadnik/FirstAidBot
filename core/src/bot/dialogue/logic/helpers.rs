use super::keyboard::make_keyboard;
use crate::{bot::prelude::*, logic::Msg, State, MAINTAINER_USERNAMES};
use teloxide::types::ParseMode::Html;

pub async fn send_state(
    bot: &FABot,
    msg: &Message,
    state: &State,
    data: &Arc<Data>,
) -> anyhow::Result<()> {
    let id = msg.chat.id;
    let Msg { link, message } = state.get_msg(data).await?;
    if let Some(link) = link {
        let link = format!("<a href='{link}'>&#8288;</a>");
        bot.send_message(id, link).parse_mode(Html).await?;
    }

    let button_texts = state.get_button_texts(data).await;
    let keyboard = make_keyboard(&button_texts?, state.lang, state.depth(), is_admin(msg));
    bot.send_message(id, &message)
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
