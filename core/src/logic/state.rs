use crate::prelude::{Data, Lang};
// use crate::prelude::State;
use anyhow::bail;
use std::fmt::Display;

#[derive(Default)]
pub struct State {
    pub lang: Lang,
    context: Vec<String>,
    pub link: Option<String>,
    pub message: String,
    pub button_texts: Vec<String>,
}

impl State {
    pub fn new(
        lang: Lang,
        context: Vec<String>,
        link: Option<String>,
        message: String,
        button_texts: Vec<String>,
    ) -> Self {
        Self {
            lang,
            context,
            link,
            message,
            button_texts,
        }
    }

    pub async fn move_to_state(&mut self, text: &str, data: &Data) -> anyhow::Result<()> {
        let mut context = self.context.clone();
        context.push(text.to_string());
        if let Ok(state) = data.get(&self.context, self.lang).await {
            *self = state;
        } else {
            bail!("No such state")
        }
        Ok(())
    }
    pub fn back(&mut self) {
        self.context.pop();
    }
    pub fn home(&mut self) {
        self.context = Vec::new();
    }
    pub fn depth(&self) -> usize {
        self.context.len()
    }
    pub fn is_empty(&self) -> bool {
        self.context.is_empty()
    }
    pub fn context(&self) -> &Vec<String> {
        &self.context
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}; {}", self.context.join("->"), self.lang)
    }
}
