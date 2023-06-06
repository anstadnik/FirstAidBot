use super::keyboard::make_keyboard;
use crate::bot::report_error::report_error;
use crate::bot::FABot;
use crate::MAINTAINER_USERNAMES;
use anyhow::{bail, Context, Result};
use first_aid_bot_core::prelude::*;
use teloxide::prelude::*;
use teloxide::types::ParseMode::Html;

pub async fn send_state(bot: &FABot, msg: &Message, ctx: &FAContext, fs: &Fs) -> Result<()> {
    let id = msg.chat.id;
    if let Some(link) = &fs.link {
        let link = format!("<a href='{link}'>&#8288;</a>");
        bot.send_message(id, link).parse_mode(Html).await?;
    }

    let keyboard = make_keyboard(
        &fs.next_states.keys().cloned().collect::<Vec<_>>(),
        ctx.lang,
        ctx.depth(),
        is_admin(msg),
    );
    bot.send_message(id, &fs.message)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub fn is_admin(msg: &Message) -> bool {
    msg.from()
        .and_then(|user| user.username.as_ref())
        .filter(|username| MAINTAINER_USERNAMES.contains(&username.as_str()))
        .is_some()
}

pub async fn get_lang_or_warn(bot: &FABot, msg: &Message, lang: &str) -> Result<Lang> {
    let ret = lang.try_into().context("Unknown language {lang}");
    if let Err(ref err) = ret {
        report_error(bot, msg.chat.id, Lang::default().details().error, err).await;
    }
    ret
}

pub async fn get_fs_or_warn<'a>(
    bot: &FABot,
    data: &'static Data,
    msg: &Message,
    ctx: &FAContext,
) -> Result<Cfs<'a>> {
    let d = data.get().await?;
    let s = d.get_state(ctx);
    let ret = match s {
        Ok(s) => s,
        Err(e) => {
            report_error(bot, msg.chat.id, Lang::default().details().error, &e).await;
            bail!(e)
        }
    };
    // .report_if_err(bot, msg.chat.id, Lang::default().details().error)
    // .await;
    Ok(ret)
}
