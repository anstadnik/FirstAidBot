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
        let state = if let Some(data) = &self.data {
            Cow::Borrowed(data.get(&lang).ok_or_else(map_err)?.get_state(ctx)?)
        } else {
            let data = get_data(None).await?;
            Cow::Owned(data.get(&lang).ok_or_else(map_err)?.get_state(ctx)?.to_owned())
        };
        Ok(state)
    }
}
