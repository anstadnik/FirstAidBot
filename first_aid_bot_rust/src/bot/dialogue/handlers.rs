use crate::bot::keyboard::make_keyboard_from_state;

use super::prelude::*;
use redis::aio::MultiplexedConnection;
use teloxide::types::ParseMode;

pub async fn reset_dialogue(
    bot: FirstAirBot,
    msg: Message,
    data: Arc<Data>,
    mut redis_con: MultiplexedConnection,
    dialogue: FirstAidDialogue,
    lang: String,
) -> anyhow::Result<()> {
    let lang = get_lang_or_warn_and_default(&bot, &msg, lang).await?;
    let state = data.get(lang, &[]).await?;
    log_to_redis(&msg, &mut redis_con, &lang, &Vec::new()).await;
    let keyboard = make_keyboard_from_state(&state, lang, &[]);
    send_state(&bot, &msg, &state, lang, keyboard).await?;
    let new_dialogue = State::Dialogue {
        lang: lang.name(),
        context: Vec::new(),
    };
    dialogue.update(new_dialogue).await?;
    Ok(())
}

pub async fn handle_dialogue(
    bot: FirstAirBot,
    msg: Message,
    dialogue: FirstAidDialogue,
    data: Arc<Data>,
    mut redis_con: MultiplexedConnection,
    (lang, mut context): (String, Vec<String>),
) -> anyhow::Result<()> {
    let lang = get_lang_or_warn_and_default(&bot, &msg, lang).await?;
    let state = &data.get(lang, &context).await?;
    log_to_redis(&msg, &mut redis_con, &lang, &context).await;
    match msg.text() {
        Some(text) if text == lang.details().button_home => {
            reset_dialogue(bot, msg, data, redis_con, dialogue, lang.name()).await?;
        }
        Some(text) if text == lang.details().button_back => {
            context.pop();
            move_to_state(bot, msg, dialogue, data, redis_con, context, lang).await?;
        }
        Some(text)
            if context.is_empty()
                && Lang::iter().any(|lang| lang.details().button_lang_name == text) =>
        {
            let lang = Lang::iter()
                .find(|lang| lang.details().button_lang_name == text)
                .unwrap();
            reset_dialogue(bot, msg, data, redis_con, dialogue, lang.name()).await?;
        }
        Some(text) if state.next_states.contains_key(&text.to_string()) => {
            context.push(text.to_string());
            move_to_state(bot, msg, dialogue, data, redis_con, context, lang).await?;
        }
        _ => {
            let keyboard = make_keyboard_from_state(state, lang, &context);
            #[allow(deprecated)]
            bot.send_message(msg.chat.id, lang.details().use_buttons_text)
                .parse_mode(ParseMode::Markdown)
                .reply_markup(keyboard)
                .await?;
        }
    }

    Ok(())
}
