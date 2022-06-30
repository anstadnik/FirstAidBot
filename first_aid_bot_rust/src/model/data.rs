use super::prelude::*;
use anyhow::Error;
use std::borrow::Cow;

#[derive(Debug)]
pub struct Data {
    data: Option<MultilangStates>,
}

impl Data {
    pub fn dynamic() -> Self {
        Self { data: None }
    }
    pub async fn cached() -> Self {
        Self {
            data: Some(get_data().await.unwrap()),
        }
    }
    pub async fn get<'a>(&'a self, lang: Lang, context: &'_ [String]) -> anyhow::Result<Cow<'a, FS>> {
        let state = match &self.data {
            Some(data) => Cow::Borrowed(
                data.get(&lang)
                    .ok_or_else(|| Error::msg("No such lang {lang}"))?
                    .get_state(context)?,
            ),
            None => Cow::Owned(
                get_data()
                    .await?
                    .get(&lang)
                    .ok_or_else(|| Error::msg("No such lang {lang}"))?
                    .get_state(context)?
                    .to_owned(),
            ),
        };
        Ok(state)
    }
}
