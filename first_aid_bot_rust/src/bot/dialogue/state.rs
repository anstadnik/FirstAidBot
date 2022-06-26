use crate::bot::keyboard::make_keyboard_from_state;

use super::prelude::*;
use redis::aio::MultiplexedConnection;
use std::sync::Arc;
use teloxide::types::Message;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum State {
    Start { lang: String },
    Dialogue { lang: String, context: Vec<String> },
}

impl Default for State {
    fn default() -> Self {
        Self::Start {
            lang: Lang::default().name(),
        }
    }
}

pub async fn move_to_state(
    bot: FirstAirBot,
    msg: Message,
    dialogue: FirstAidDialogue,
    data: Arc<Data>,
    redis_con: MultiplexedConnection,
    context: Vec<String>,
    lang: Lang,
) -> anyhow::Result<()> {
    let state = &data.get(lang, &context).await?;
    // let state = get_state(state, &context)?;
    let keyboard = make_keyboard_from_state(state, lang, &context);
    send_state(&bot, &msg, state, lang, keyboard).await?;
    if state.next_states.is_empty() {
        return reset_dialogue(bot, msg, data, redis_con, dialogue, lang.name()).await;
    }
    dialogue
        .update(State::Dialogue {
            lang: lang.name(),
            context,
        })
        .await?;
    Ok(())
}
