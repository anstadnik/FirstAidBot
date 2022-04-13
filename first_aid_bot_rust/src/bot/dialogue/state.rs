use super::{reset_dialogue, FirstAidDialogue};
use crate::bot::helpers::{get_state, send_message, ExtraKeys};
use crate::bot::{dialogue::handle_dialogue, Data};
use crate::lang::Lang;
use redis::aio::MultiplexedConnection;
use std::sync::Arc;
use teloxide::adaptors::{AutoSend, DefaultParseMode};
use teloxide::{macros::DialogueState, types::Message, Bot};

#[derive(DialogueState, Clone, serde::Serialize, serde::Deserialize)]
#[handler_out(anyhow::Result<()>)]
pub enum State {
    #[handler(reset_dialogue)]
    Start { lang: Lang },

    #[handler(handle_dialogue)]
    Dialogue { lang: Lang, context: Vec<String> },
}

impl Default for State {
    fn default() -> Self {
        Self::Start {
            lang: Lang::default()
        }
    }
}

pub async fn move_to_state(
    bot: AutoSend<DefaultParseMode<Bot>>,
    msg: Message,
    dialogue: FirstAidDialogue,
    data: Arc<Data>,
    redis_con: MultiplexedConnection,
    context: Vec<String>,
    lang: Lang,
) -> anyhow::Result<()> {
    let state = &data.get().await?[&lang];
    let state = get_state(state, &context);
    send_message(&bot, &msg, state, ExtraKeys::new(&context, None)).await?;
    if state.options.is_none() {
        return reset_dialogue(bot, msg, data, redis_con, dialogue, (lang.name(),)).await;
    }
    dialogue.update(State::Dialogue { lang, context }).await?;
    Ok(())
}
