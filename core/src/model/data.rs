use crate::model::MultilangFs;
use crate::prelude::*;
use anyhow::{anyhow, Result};
use log::info;
use std::borrow::Cow;

#[derive(Debug)]
pub struct Data {
    data: Option<MultilangFs>,
}

impl Data {
    pub fn dynamic() -> Self {
        info!("Dynamic data!");
        Self { data: None }
    }
    pub fn cached() -> Result<Self> {
        info!("Cached data!");
        Ok(Self {
            data: Some(get_data_from_file("table.csv")?),
        })
    }
    pub async fn get(&self) -> Result<Cow<MultilangFs>> {
        Ok(match self.data {
            Some(ref data) => Cow::Borrowed(data),
            None => Cow::Owned(get_data_from_web().await?),
        })
    }
}

pub type Cfs<'a> = Cow<'a, Fs>;

pub trait CowMultLangFsExt<'a> {
    fn get_state(self, ctx: &FAContext) -> Result<Cfs<'a>>;
}

impl<'a> CowMultLangFsExt<'a> for Cow<'a, MultilangFs> {
    fn get_state(self, ctx: &FAContext) -> Result<Cfs<'a>> {
        let err_lang = anyhow!("No such lang: {}", ctx.lang);
        let err_ctx = |key| move || anyhow!("Cannot find {key} for context {:?}", ctx.context);
        match self {
            Cow::Borrowed(v) => {
                Ok(Cow::Borrowed(ctx.context.iter().try_fold(
                    v.get(&ctx.lang).ok_or(err_lang)?,
                    |fs, key| fs.next_states.get(key).ok_or_else(err_ctx(key)),
                )?))
            }
            Cow::Owned(mut v) => {
                Ok(Cow::Owned(ctx.context.iter().try_fold(
                    v.remove(&ctx.lang).ok_or(err_lang)?,
                    |mut fs, key| fs.next_states.remove(key).ok_or_else(err_ctx(key)),
                )?))
            }
        }
    }
}
