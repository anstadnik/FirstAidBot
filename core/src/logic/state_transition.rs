use std::borrow::Cow;

use anyhow::{bail, Context};

use crate::model::FS;

use super::prelude::*;

#[derive(Default)]
pub struct State {
    pub lang: Lang,
    context: Vec<String>,
}

pub struct Msg {
    pub link: Option<String>,
    pub message: String,
}

impl State {
    pub fn new(lang: Lang, context: Vec<String>) -> Self {
        Self { lang, context }
    }

    pub async fn move_to_state(&mut self, text: &str, data: &Arc<Data>) -> anyhow::Result<()> {
        let finite_state = self.get_finite_state(data).await?;

        if finite_state.next_states.contains_key(text) {
            self.context.push(text.to_string());
        } else {
            bail!("No such state");
        }
        Ok(())
    }
    pub fn back(&mut self) {
        self.context.pop();
    }
    pub fn home(&mut self) {
        self.context = Vec::new();
    }

    pub async fn get_button_texts(&self, data: &Arc<Data>) -> anyhow::Result<Vec<String>> {
        let finite_state = self.get_finite_state(data).await?;
        Ok(finite_state.next_states.keys().cloned().collect())
    }

    pub async fn get_msg(&self, data: &Arc<Data>) -> anyhow::Result<Msg> {
        let finite_state = self.get_finite_state(data).await?;
        Ok(Msg {
            link: finite_state.link.clone(),
            message: finite_state.message.clone(),
        })
    }

    async fn get_finite_state<'a>(&'a self, data: &'a Arc<Data>) -> anyhow::Result<Cow<FS>> {
        data.get(&self)
            .await
            .context(self.lang.details().error_due_to_update)
    }

    pub fn depth(&self) -> usize {
        self.context.len()
    }
}
