use async_recursion::async_recursion;

use super::prelude::*;

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
    let keyboard = make_keyboard_from_state(state, lang, &context);
    send_state(bot, msg, state, lang, keyboard).await?;
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
