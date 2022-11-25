use super::prelude::*;
use anyhow::anyhow;
use log::info;
use std::borrow::Cow;

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
    pub async fn get<'a>(&'a self, lang: Lang, ctx: &[String]) -> anyhow::Result<Cow<'a, FS>> {
        let map_err = || anyhow!("No such lang {lang}");
        let state = match &self.data {
            Some(data) => Cow::Borrowed(data.get(&lang).ok_or_else(map_err)?.get_state(ctx)?),
            None => Cow::Owned(
                get_data(None)
                    .await?
                    .get(&lang)
                    .ok_or_else(map_err)?
                    .get_state(ctx)?
                    .clone(),
            ),
        };
        Ok(state)
    }
}
