mod data;
mod finite_state;
mod lang;

pub mod prelude {
    pub use super::data::Data;
    pub use super::finite_state::{FiniteState, FiniteStateOptions, MultilangStates};
    pub use super::get_data;
    pub use super::lang::Lang;
}

use bytes::Buf;
use csv::Reader;
use futures::{stream, StreamExt, TryStreamExt};
use prelude::*;
use std::env;

use self::finite_state::Record;

async fn get_csv_records(sheet_id: String, sheet_name: String) -> anyhow::Result<Vec<Record>> {
    let url = format!(
        "https://docs.google.com/spreadsheets/d/{}/gviz/tq?tqx=out:csv&sheet={}",
        sheet_id, sheet_name
    );
    let reader = reqwest::get(url).await?;
    let rdr = Reader::from_reader(reader.bytes().await?.reader());
    rdr.into_deserialize()
        .map(|row: Result<Record, _>| {
            let mut row = row?;
            row.hierarchy = row.hierarchy.trim().to_string();
            Ok(row)
        })
        .collect()
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
    let options: Vec<_> = data
        .iter()
        .filter(|row| match &key {
            None => !row.hierarchy.contains('.'),
            Some(parent_key) => {
                row.hierarchy.starts_with(parent_key)
                    && !row.hierarchy.replacen(parent_key, "", 1).contains('.')
            }
        })
        .collect();
    if options.is_empty() {
        return Ok(None);
    }

    let convert_row = |row: &&Record| {
        let options = fill_item(data, Some(format!("{}.", row.hierarchy)))?;
        let state = FiniteState::parse_row(row, options);
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

async fn get_finite_state(lang: Lang) -> anyhow::Result<FiniteState> {
    let sheet_id = env::var("SHEET_ID").expect("Please define a SHEET_ID env variable");
    let csv_records = get_csv_records(sheet_id, lang.name()).await?;
    Ok(FiniteState::new(
        None,
        lang.details().greeting.to_string(),
        fill_item(&csv_records, None)?,
    ))
}

pub async fn get_data() -> anyhow::Result<MultilangStates> {
    stream::iter(Lang::iter())
        .then(|lang| async move { get_finite_state(lang).await.map(|fs| (lang, fs)) })
        .try_collect()
        .await
}
