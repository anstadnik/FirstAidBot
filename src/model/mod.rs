mod data;
mod finite_state;
mod lang;

pub mod prelude {
    pub use super::data::Data;
    pub use super::finite_state::{FSNextStates, MultilangStates, FS};
    pub use super::get_data;
    pub use super::lang::Lang;
}

use std::{env, fs::File, io};

use anyhow::{anyhow, Context};
use bytes::Buf;
use csv::Reader;
use futures::{stream, StreamExt, TryStreamExt};
use prelude::*;

use self::finite_state::Row;

// TODO: Add loading of special messages for Lang

async fn get_rows(filename: Option<&str>, sheet_name: String) -> anyhow::Result<Vec<Row>> {
    let rdr: Box<dyn io::Read> = match filename {
        Some(filename) => Box::new(io::BufReader::new(File::open(filename)?)),
        None => {
            let sheet_id = env::var("SHEET_ID")?;
            let url = format!(
        "https://docs.google.com/spreadsheets/d/{sheet_id}/gviz/tq?tqx=out:csv&sheet={sheet_name}"
    );
            Box::new(reqwest::get(url).await?.bytes().await?.reader())
        }
    };
    let rdr = Reader::from_reader(rdr);
    rdr.into_deserialize()
        .collect::<Result<_, _>>()
        .context("Cannot parse csv")
}
fn get_next_states_for_key(data: &[Row], key: &str) -> anyhow::Result<FSNextStates> {
    data.iter()
        .filter(|row| {
            row.key
                .strip_prefix(key)
                .map_or(false, |s| !s.contains('.'))
        })
        .map(|mut row| {
            if let Some(key) = row.question.strip_prefix('#') {
                row = data
                    .iter()
                    .find(|row_| row_.key == key)
                    .ok_or_else(|| anyhow!("Didn't find {} in row {}", row.question, row.key))?;
            };
            let key = row.key.clone() + ".";
            let next_states = get_next_states_for_key(data, &key)?;
            Ok((
                row.question.to_owned(),
                FS::parse_row(row, next_states)
                    .with_context(|| format!("Error in parsing row with key {}", row.key))?,
            ))
        })
        .collect()
}

async fn get_finite_state(lang: Lang, filename: Option<&str>) -> anyhow::Result<FS> {
    let mut rows = get_rows(filename, lang.name()).await?;
    rows.retain(|record| !record.is_empty());
    for row in &mut rows {
        row.key = row.key.trim().to_string();
    }
    Ok(FS::entry(&lang, get_next_states_for_key(&rows, "")?))
}

pub async fn get_data(filename: Option<&str>) -> anyhow::Result<MultilangStates> {
    stream::iter(Lang::iter())
        .then(|lang| async move { get_finite_state(lang, filename).await.map(|fs| (lang, fs)) })
        .try_collect()
        .await
}

#[cfg(test)]
mod test;
