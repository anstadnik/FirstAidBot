use super::prelude::*;
use anyhow::anyhow;
use async_recursion::async_recursion;
use teloxide::types::ParseMode;

#[async_recursion]
pub async fn move_to_state(
    args: &FAMsgArgs,
    context: Vec<String>,
    lang: Lang,
) -> anyhow::Result<()> {
    let FAMsgArgs {
        bot,
        msg,
        dialogue,
        data,
        ..
    } = args;
    let state = &data.get(lang, &context).await?;
    log_to_redis(args, &lang, &context).await;
    send_state(bot, msg.chat.id, state, lang, &context).await?;
    if state.next_states.is_empty() {
        return move_to_state(args, Vec::new(), lang).await;
    }
    let new_dialogue = State::Dialogue {
        lang: lang.name(),
        context,
    };
    dialogue.update(new_dialogue).await?;
    Ok(())
}

pub async fn state_transition(
    args: &FAMsgArgs<'_>,
    context: Vec<String>,
    lang: Lang,
) -> anyhow::Result<()> {
    let state = &args.data.get(lang, &context).await?;
    log_to_redis(args, &lang, &context).await;
    let mut context = context.clone();
    match args.msg.text() {
        Some(text) if text == lang.details().button_home => {
            move_to_state(args, Vec::new(), lang).await?;
        }
        Some(text) if text == lang.details().button_back => {
            context.pop();
            move_to_state(args, context, lang).await?;
        }
        Some(text)
            if context.is_empty()
                && Lang::iter().any(|lang| lang.details().button_lang_name == text) =>
        {
            let lang = Lang::iter()
                .find(|lang| lang.details().button_lang_name == text)
                .ok_or_else(|| anyhow!("Wrong language WTF?"))?;
            move_to_state(args, Vec::new(), lang).await?;
        }
        Some(text) if state.next_states.contains_key(&text.to_string()) => {
            context.push(text.to_string());
            move_to_state(args, context, lang).await?;
        }
        _ => {
            let keyboard = make_keyboard_from_state(state, lang, &context);
            #[allow(deprecated)]
            args.bot
                .send_message(args.msg.chat.id, lang.details().use_buttons_text)
                .parse_mode(ParseMode::Markdown)
                .reply_markup(keyboard)
                .await?;
        }
    }
    anyhow::Ok(())
}
