use std::sync::Arc;

use teloxide::{dispatching2::dialogue::InMemStorage, macros::DialogueState, prelude2::*};

use crate::{
    bot::helpers::{get_state, make_keyboard, send_message, GO_BACK_TEXT, GO_TO_BEGINNING_TEXT},
    model::{FiniteState, FiniteStateOptions},
};

pub type FirstAidDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(DialogueState, Clone)]
#[handler_out(anyhow::Result<()>)]
pub enum State {
    #[handler(reset_dialogue)]
    Start,

    #[handler(handle_dialogue)]
    Dialogue { context: Vec<String> },
}

impl Default for State {
    fn default() -> Self {
        Self::Start
    }
}

pub async fn reset_dialogue(
    bot: AutoSend<Bot>,
    msg: Message,
    data: Arc<FiniteState>,
    dialogue: FirstAidDialogue,
) -> anyhow::Result<()> {
    log::debug!("Handling start! for {}, {}", msg.id, msg.from().unwrap().id);
    send_message(&bot, &msg, &data).await?;
    dialogue.update(State::Dialogue { context: vec![] }).await?;
    Ok(())
}

async fn move_to_state(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: FirstAidDialogue,
    data: Arc<FiniteState>,
    context: Vec<String>,
) -> anyhow::Result<()> {
    let state = get_state(data.as_ref(), &context).await;
    send_message(&bot, &msg, state).await?;
    if state.options.is_none() {
        return reset_dialogue(bot, msg, data, dialogue).await;
    }
    dialogue.update(State::Dialogue { context }).await?;
    Ok(())
}

async fn handle_dialogue(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: FirstAidDialogue,
    data: Arc<FiniteState>,
    (mut context,): (Vec<String>,),
) -> anyhow::Result<()> {
    log::debug!("Handling a dialogue!");
    let FiniteStateOptions { ordered_keys, .. } =
        get_state(data.as_ref(), &context).await.options.as_ref().unwrap();
    log::debug!("Got a message {:?} ({:?})", msg.text(), ordered_keys);
    match msg.text() {
        Some(GO_TO_BEGINNING_TEXT) => {
            reset_dialogue(bot, msg, data, dialogue).await?;
        }
        Some(GO_BACK_TEXT) => {
            context.pop();
            move_to_state(bot, msg, dialogue, data, context).await?;
        }
        Some(text) if ordered_keys.contains(&text.to_string()) => {
            context.push(text.to_string());
            move_to_state(bot, msg, dialogue, data, context).await?;
        }
        _ => {
            let keyboard = make_keyboard(ordered_keys).await;
            bot.send_message(msg.chat.id, "Використайте кнопки")
                .reply_markup(keyboard)
                .await?;
        }
    }

    Ok(())
}
