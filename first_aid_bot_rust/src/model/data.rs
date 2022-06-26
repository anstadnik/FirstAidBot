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
    pub async fn get(&self, lang: Lang, context: &[String]) -> anyhow::Result<Cow<FS>> {
        Ok(match &self.data {
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
        })
    }
}
