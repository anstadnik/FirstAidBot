use std::sync::Arc;

use teloxide::{prelude2::*, utils::command::BotCommand};

use crate::model::FiniteState;

use super::dialogue::{reset_dialogue, FirstAidDialogue};

#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "FirstAidBot")]
pub enum FirstAidCommands {
    #[command(description = "Restart the bot")]
    Start,
}

pub async fn commands_handler(
    msg: Message,
    bot: AutoSend<Bot>,
    cmd: FirstAidCommands,
    data: Arc<FiniteState>,
    dialogue: FirstAidDialogue,
) -> anyhow::Result<()> {
    let _ = match cmd {
        FirstAidCommands::Start => {
            dialogue.exit().await?;
            return reset_dialogue(bot, msg, data, dialogue).await;
        }
    };
}
