use super::{reset_dialogue, FirstAidDialogue};
use crate::{
    bot::{
        dialogue::{handle_dialogue, setup},
        helpers::{get_state, send_message, ExtraKeys},
        MultilangStates,
    },
    Lang, LANGS,
};
use redis::aio::MultiplexedConnection;
use std::sync::Arc;
use teloxide::{
    adaptors::{AutoSend, DefaultParseMode},
    macros::DialogueState,
    types::Message,
    Bot,
};

#[derive(DialogueState, Clone, serde::Serialize, serde::Deserialize)]
#[handler_out(anyhow::Result<()>)]
pub enum State {
    #[handler(reset_dialogue)]
    Start { lang: String },

    #[handler(setup)]
    ChangeLang,

    #[handler(handle_dialogue)]
    Dialogue { lang: String, context: Vec<String> },
}

impl Default for State {
    fn default() -> Self {
        Self::Start {
            lang: LANGS[0].name.to_string(),
        }
    }
}

pub async fn move_to_state(
    bot: AutoSend<DefaultParseMode<Bot>>,
    msg: Message,
    dialogue: FirstAidDialogue,
    data: Arc<MultilangStates>,
    redis_con: MultiplexedConnection,
    context: Vec<String>,
    lang: Lang,
) -> anyhow::Result<()> {
    let state = get_state(&data[&lang], &context).await;
    send_message(&bot, &msg, state, ExtraKeys::new(&context)).await?;
    if state.options.is_none() {
        return reset_dialogue(bot, msg, data, redis_con, dialogue, lang).await;
    }
    let lang = lang.name.to_string();
    dialogue.update(State::Dialogue { lang, context }).await?;
    Ok(())
}