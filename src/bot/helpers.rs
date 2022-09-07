use super::prelude::*;
use itertools::Itertools;
use teloxide::utils::markdown::escape;

fn split_msg(msg: &str) -> impl Iterator<Item = String> {
    let mut ret: Vec<String> = Vec::new();
    for msg in msg.split_inclusive('\n') {
        if ret.is_empty() || ret.last().unwrap().len() + msg.len() >= 4000 {
            ret.push(msg.to_string());
        } else {
            *ret.last_mut().unwrap() += msg;
        }
    }
    ret.into_iter().flat_map(|msg: String| -> Vec<_> {
        msg.chars()
            .chunks(4000)
            .into_iter()
            .map(|c| c.collect())
            .collect()
    })
}

pub async fn send_plain_string(bot: &FABot, id: ChatId, msg: &str) -> anyhow::Result<()> {
    for msg in split_msg(msg) {
        let msg = "```".to_string() + &escape(&msg) + "```";
        bot.send_message(id, msg).await?;
    }
    Ok(())
}
