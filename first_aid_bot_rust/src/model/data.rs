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
            data: Some(get_data().await),
        }
    }
    pub async fn get<'a>(&'a self) -> BoxOrBorrow<'a> {
        match &self.data {
            Some(data) => BoxOrBorrow::Borrowed(data),
            None => BoxOrBorrow::Owned(get_data().await),
        }
    }
}
