use crate::{model::FiniteState, FiniteStateMsg, FiniteStateOptions};
use std::sync::Arc;
use teloxide::{
    dispatching2::dialogue::InMemStorage,
    macros::DialogueState,
    payloads::SendMessageSetters,
    prelude2::*,
    types::{KeyboardButton, KeyboardMarkup},
};

type MyDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(DialogueState, Clone)]
#[handler_out(anyhow::Result<()>)]
pub enum State {
    #[handler(handle_start)]
    Start,

    #[handler(handle_dialogue)]
    Dialogue { context: Vec<String> },
}

impl Default for State {
    fn default() -> Self {
        Self::Start
    }
}

async fn get_current_options<'a>(
    data: &'a FiniteState,
    context: &Vec<String>,
) -> &'a FiniteStateOptions {
    let mut current_state = data;
    for choise in context {
        current_state = &current_state.options.as_ref().unwrap()[choise];
    }
    &current_state.options
}

async fn make_keyboard(key_texts: Vec<String>) -> KeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    for key_texts in key_texts.chunks(2) {
        let row = key_texts
            .iter()
            .map(|key_text| KeyboardButton::new(key_text))
            .collect();

        keyboard.push(row);
    }

    KeyboardMarkup::new(keyboard)
}

async fn send_message(
    bot: &AutoSend<Bot>,
    msg: &Message,
    message: &FiniteStateMsg,
    key_texts: Vec<String>,
) -> anyhow::Result<()> {
    if let Some(link) = &message.link {
        // TODO: Hide the link <28-02-22, astadnik> //
        bot.send_message(msg.chat.id, link).await?;
    }
    let keyboard = make_keyboard(key_texts).await;
    bot.send_message(msg.chat.id, &message.message)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

async fn handle_start(
    bot: AutoSend<Bot>,
    msg: Message,
    data: Arc<FiniteState>,
    dialogue: MyDialogue,
) -> anyhow::Result<()> {
    let options = get_current_options(data.as_ref(), &vec![])
        .await
        .as_ref()
        .unwrap();
    send_message(
        &bot,
        &msg,
        &data.as_ref().message,
        options.keys().cloned().collect(),
    )
    .await?;
    dialogue.update(State::Dialogue { context: vec![] }).await?;
    Ok(())
}

async fn handle_dialogue(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
    data: Arc<FiniteState>,
    (mut context,): (Vec<String>,),
) -> anyhow::Result<()> {
    let options = get_current_options(data.as_ref(), &context)
        .await
        .as_ref()
        .unwrap();
    match msg.text() {
        Some(text) if options.contains_key(&text.to_string()) => {
            let state = &options[text];

            send_message(
                &bot,
                &msg,
                &state.message,
                options.keys().cloned().collect(),
            )
            .await?;
            // bot.send_message(msg.chat.id, state.message).await?;
            context.push(text.to_string());
            dialogue.update(State::Dialogue { context }).await?;
        }
        _ => {
            let keyboard = make_keyboard(options.keys().cloned().collect()).await;
            bot.send_message(msg.chat.id, "Використайте кнопки")
                .reply_markup(keyboard)
                .await?;
        }
    }

    Ok(())
}
