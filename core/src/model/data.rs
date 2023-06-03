// use crate::model::{MultilangStates, State};
// use crate::Context;

// use super::prelude::*;
use crate::model::MultilangStates;
use crate::prelude::{get_data, Lang, State};
// use crate::prelude::Context;
// use crate::prelude::State;
use anyhow::anyhow;
use log::info;

#[derive(Debug)]
pub struct Data {
    data: Option<MultilangStates>,
}

impl Data {
    pub fn dynamic() -> Self {
        info!("Dynamic data!");
        Self { data: None }
    }
    pub async fn cached() -> Self {
        info!("Cached data!");
        let data = Some(get_data(Some("table.csv")).await.unwrap());
        Self { data }
    }
    pub async fn get(&self, ctx: &[String], lang: Lang) -> anyhow::Result<State> {
        // let Context { lang, context: ctx } = state;
        let map_err = || anyhow!("No such lang {lang}");
        let (link, message, button_texts) = if let Some(ref data) = self.data {
            let fs = data.get(&lang).ok_or_else(map_err)?.get_state(ctx)?;
            (
                fs.link.clone(),
                fs.message.clone(),
                fs.next_states.keys().cloned().collect(),
            )
        } else {
            let tmp = get_data(None).await?.remove(&lang).ok_or_else(map_err)?;
            let fs = tmp.get_state(ctx)?;
            (
                fs.link.clone(),
                fs.message.clone(),
                fs.next_states.keys().cloned().collect(),
            )
        };

        Ok(State::new(lang, ctx.to_vec(), link, message, button_texts))
    }
}
