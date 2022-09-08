use super::helpers::{log_to_redis, send_state};
use crate::bot::prelude::*;
use anyhow::{anyhow, Context};
use async_recursion::async_recursion;
use redis::aio::MultiplexedConnection;
use teloxide::utils::markdown::{escape, escape_code};

pub struct FALogic<'a> {
    pub bot: &'a FABot,
    pub msg: &'a Message,
    pub dialogue: &'a FADialogue,
    pub data: &'a Arc<Data>,
}

impl<'a> FALogic<'a> {
    pub const fn new(
        bot: &'a FABot,
        msg: &'a Message,
        dialogue: &'a FADialogue,
        data: &'a Arc<Data>,
    ) -> Self {
        Self {
            bot,
            msg,
            dialogue,
            data,
        }
    }
}

impl FALogic<'_> {
    #[async_recursion]
    pub async fn move_to_state(
        &self,
        context: Vec<String>,
        lang: Lang,
        redis_con: MultiplexedConnection,
    ) -> anyhow::Result<()> {
        let FALogic {
            bot,
            msg,
            dialogue,
            data,
            ..
        } = self;
        let state = &data.get(lang, &context).await?;
        log_to_redis(self.msg, &lang, &context, redis_con.clone()).await;
        send_state(bot, msg.chat.id, state, lang, &context).await?;
        if state.next_states.is_empty() {
            return self.move_to_state(Vec::new(), lang, redis_con).await;
        }
        let lang = lang.name();
        dialogue.update(State::Dialogue { lang, context }).await?;
        Ok(())
    }

    pub async fn state_transition(
        &self,
        mut context: Vec<String>,
        lang: Lang,
        redis_con: MultiplexedConnection,
    ) -> anyhow::Result<()> {
        let state = &match self.data.get(lang, &context).await {
            Ok(it) => it,
            Err(_) => {
                self.bot
                    .send_message(
                        self.msg.chat.id,
                        escape_code(lang.details().error_due_to_update),
                    )
                    .await?;
                return self.move_to_state(Vec::new(), lang, redis_con).await;
            }
        };
        log_to_redis(self.msg, &lang, &context, redis_con.clone()).await;
        match self.msg.text() {
            Some(text) if text == lang.details().button_home => {
                self.move_to_state(Vec::new(), lang, redis_con).await?;
            }
            Some(text) if text == lang.details().button_back => {
                context.pop();
                self.move_to_state(context, lang, redis_con).await?;
            }
            Some(text)
                if context.is_empty()
                    && Lang::iter().any(|lang| lang.details().button_lang_name == text) =>
            {
                let lang = Lang::iter()
                    .find(|lang| lang.details().button_lang_name == text)
                    .ok_or_else(|| anyhow!("Wrong language WTF?"))?;
                self.move_to_state(Vec::new(), lang, redis_con).await?;
            }
            Some(text) if state.next_states.contains_key(&text.to_string()) => {
                context.push(text.to_string());
                self.move_to_state(context.clone(), lang, redis_con)
                    .await
                    .with_context(|| format!("Error while moving into context {context:?}"))?;
            }
            _ => {
                self.bot
                    .send_message(self.msg.chat.id, escape(lang.details().use_buttons_text))
                    .await?;
                self.move_to_state(context, lang, redis_con).await?;
            }
        }
        anyhow::Ok(())
    }
}
