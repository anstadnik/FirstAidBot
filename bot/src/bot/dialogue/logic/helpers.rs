use super::keyboard::make_keyboard;
use crate::bot::FABot;
use crate::MAINTAINER_USERNAMES;
use anyhow::Result;
use first_aid_bot_core::prelude::*;
use teloxide::prelude::*;
use teloxide::types::ParseMode::Html;

pub async fn send_state(bot: &FABot, msg: &Message, ctx: &FAContext, fs: &Fs) -> Result<()> {
    let id = msg.chat.id;
    if let Some(link) = &fs.link {
        let link = format!("<a href='{link}'>&#8288;</a>");
        bot.send_message(id, link).parse_mode(Html).await?;
    }

    let keys = fs.next_states.keys().map(AsRef::as_ref).collect::<Vec<_>>();
    let kbd = make_keyboard(&keys, ctx.lang, ctx.depth(), is_admin(msg));
    bot.send_message(id, &fs.message).reply_markup(kbd).await?;
    Ok(())
}

pub fn is_admin(msg: &Message) -> bool {
    msg.from()
        .and_then(|user| user.username.as_ref())
        .is_some_and(|username| MAINTAINER_USERNAMES.contains(&username.as_str()))
}
