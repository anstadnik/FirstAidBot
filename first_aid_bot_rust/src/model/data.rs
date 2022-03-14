use super::prelude::*;
use std::ops::Deref;

pub enum BoxOrBorrow<'a> {
    Owned(MultilangStates),
    Borrowed(&'a MultilangStates),
}

impl<'a> Deref for BoxOrBorrow<'a> {
    type Target = MultilangStates;

    fn deref(&self) -> &Self::Target {
        match self {
            BoxOrBorrow::Owned(b) => b,
            BoxOrBorrow::Borrowed(b) => b,
        }
    }
}

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
    pub async fn get(&self) -> anyhow::Result<BoxOrBorrow<'_>> {
        match &self.data {
            Some(data) => Ok(BoxOrBorrow::Borrowed(data)),
            None => Ok(BoxOrBorrow::Owned(get_data().await?)),
        }
    }
}
