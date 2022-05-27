use super::prelude::*;
use anyhow::Context;
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

pub fn get_state<'a>(
    state: &'a FiniteState,
    context: &[String],
) -> anyhow::Result<&'a FiniteState> {
    let mut current_state = state;
    for choise in context {
        current_state = current_state.get_next_state(choise).context(format!(
            "Cannot find next state: {state:?} being on {context:?}"
        ))?;
    }
    Ok(current_state)
}

pub async fn move_to_state(
    bot: FirstAidBot,
    msg: Message,
    dialogue: FirstAidDialogue,
    data: Arc<Data>,
    redis_con: MultiplexedConnection,
    context: Vec<String>,
    lang: Lang,
) -> anyhow::Result<()> {
    let state = &data.get().await?[&lang];
    let state = get_state(state, &context)?;
    let keyboard = make_keyboard(state.get_options(), lang, &context);
    send_state(&bot, &msg, state, lang, keyboard).await?;
    if state.get_options().is_empty() {
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
