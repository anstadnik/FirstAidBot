mod data;
mod finite_state;

pub mod prelude {
    pub use super::data::Data;
    pub use super::finite_state::{FiniteState, FiniteStateOptions, MultilangStates};
    pub use super::get_data;
}

use self::finite_state::Record;
use crate::{Lang, LANGS, SHEET_ID};
use bytes::Buf;
use csv::Reader;
use futures::{stream, StreamExt, TryStreamExt};
use prelude::*;

async fn get_csv(sheet_id: &str, sheet_name: &str) -> anyhow::Result<Vec<Record>> {
    let url = format!(
        "https://docs.google.com/spreadsheets/d/{sheet_id}/gviz/tq?tqx=out:csv&sheet={sheet_name}",
    );
    let reader = reqwest::get(url).await?;
    let rdr = Reader::from_reader(reader.bytes().await?.reader());
    rdr.into_deserialize().map(|row| Ok(row?)).collect()
}

fn get_ordered_keys(options: &[&Record], key: Option<String>) -> anyhow::Result<Vec<String>> {
    let key = key.unwrap_or_default();
    let mut order = options
        .iter()
        .map(|row| {
            Ok((
                row.hierarchy.replace(&key, "").parse()?,
                row.option.to_owned(),
            ))
        })
        .collect::<anyhow::Result<Vec<(u16, _)>>>()?;
    order.sort();
    Ok(order.into_iter().map(|x| x.1).collect())
}

fn fill_item(data: &[Record], key: Option<String>) -> anyhow::Result<Option<FiniteStateOptions>> {
    let predicate: Box<dyn Fn(&&Record) -> bool> = match &key {
        None => Box::new(|row| !row.hierarchy.contains('.')),
        Some(parent_key) => Box::new(move |row| {
            row.hierarchy.starts_with(parent_key)
                && !row.hierarchy.replacen(parent_key, "", 1).contains('.')
        }),
    };
    let options: Vec<_> = data.iter().filter(predicate).collect();
    if options.is_empty() {
        return Ok(None);
    }

    let convert_row = |row: &&Record| {
        let options = fill_item(data, Some(format!("{}.", row.hierarchy)))?;
        let state = FiniteState::new(row, options);
        Ok((row.option.to_owned(), state))
    };
    let ordered_keys = get_ordered_keys(&options, key)?;
    let next_states = options
        .iter()
        .map(convert_row)
        .collect::<anyhow::Result<_>>()?;
    Ok(Some(FiniteStateOptions {
        ordered_keys,
        next_states,
    }))
}

async fn get_finite_state(lang: &Lang) -> anyhow::Result<FiniteState> {
    Ok(FiniteState {
        link: None,
        message: lang.greet.to_string(),
        options: fill_item(&get_csv(SHEET_ID, lang.name).await?, None)?,
    })
}

pub async fn get_data() -> anyhow::Result<MultilangStates> {
    stream::iter(LANGS.iter())
        .then(|lang| async { Ok((lang.name.to_string(), get_finite_state(lang).await?)) })
        .try_collect()
        .await
}
