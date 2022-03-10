use crate::{
    bot::{
        dialogue::FirstAidDialogue,
        helpers::{make_keyboard, ExtraKeys},
        MultilangStates,
    },
    LANGS,
};
use redis::aio::MultiplexedConnection;
use std::sync::Arc;
use teloxide::Bot;
use teloxide::{
    adaptors::{AutoSend, DefaultParseMode},
    payloads::SendMessageSetters,
    prelude::Requester,
    types::Message,
};

use super::State;

pub async fn setup(
    bot: AutoSend<DefaultParseMode<Bot>>,
    msg: Message,
    _data: Arc<MultilangStates>,
    _redis_con: MultiplexedConnection,
    dialogue: FirstAidDialogue,
) -> anyhow::Result<()> {
    if let Some(text) = msg.text() {
        for lang in &LANGS {
            if text == lang.text {
                let lang = lang.name.to_string();
                dialogue.update(State::Start { lang }).await?;
                return Ok(());
            }
        }
    };
    let keyboard = make_keyboard(
        &LANGS
            .iter()
            .map(|lang| lang.text.to_string())
            .collect::<Vec<_>>(),
        ExtraKeys::empty(),
    )
    .await;
    bot.send_message(msg.chat.id, "Виберіть мову за допомогою кнопок")
        .reply_markup(keyboard)
        .await?;
    Ok(())
}
