mod data;
mod finite_state;
mod lang;

pub mod prelude {
    pub use super::data::Data;
    pub use super::finite_state::{FSNextStates, MultilangStates, FS};
    pub use super::get_data;
    pub use super::lang::Lang;
}

use bytes::Buf;
use csv::Reader;
use futures::{stream, StreamExt, TryStreamExt};
use prelude::*;
use std::{collections::BTreeMap, env};

use self::finite_state::Row;

async fn get_rows(sheet_id: String, sheet_name: String) -> anyhow::Result<Vec<Row>> {
    let url = format!(
        "https://docs.google.com/spreadsheets/d/{sheet_id}/gviz/tq?tqx=out:csv&sheet={sheet_name}"
    );
    let reader = reqwest::get(url).await?;
    let rdr = Reader::from_reader(reader.bytes().await?.reader());
    Ok(rdr.into_deserialize().collect::<Result<Vec<_>, _>>()?)
}

fn get_next_states_for_key(data: &[Row], key: &str) -> anyhow::Result<FSNextStates> {
    data.iter()
        .filter_map(|row| {
            (row.hierarchy.starts_with(key) && !row.hierarchy.replacen(key, "", 1).contains('.'))
                .then(|| {
                    let next_states =
                        get_next_states_for_key(data, &(row.hierarchy.clone() + "."))?;
                    Ok((row.question.to_owned(), FS::parse_row(row, next_states)?))
                })
        })
        .collect::<anyhow::Result<BTreeMap<String, FS>>>()
}

async fn get_finite_state(lang: Lang) -> anyhow::Result<FS> {
    let sheet_id = env::var("SHEET_ID").expect("Please define a SHEET_ID env variable");
    let mut rows = get_rows(sheet_id, lang.name()).await?;
    rows.retain(|record| !record.is_empty());
    rows.iter_mut().for_each(|row| {
        row.hierarchy = row.hierarchy.trim().to_string();
    });
    Ok(FS::entry(&lang, get_next_states_for_key(&rows, "")?))
}

pub async fn get_data() -> anyhow::Result<MultilangStates> {
    stream::iter(Lang::iter())
        .then(|lang| async move { get_finite_state(lang).await.map(|fs| (lang, fs)) })
        .try_collect()
        .await
}
