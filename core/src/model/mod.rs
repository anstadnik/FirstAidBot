mod data;
mod finite_state;
mod lang;

pub mod prelude {
    pub use super::data::Data;
    pub use super::get_data;
    pub use super::lang::Lang;
}

use anyhow::{anyhow, Context};
use bytes::Buf;
use csv::Reader;
use finite_state::{FSNextStates, MultilangStates};
pub use finite_state::FS;
use futures::{stream, StreamExt, TryStreamExt};
use prelude::*;
use std::{env, fs::File, io};

use self::finite_state::Row;

// TODO: Add loading of special messages for Lang

async fn get_rows(filename: Option<&str>, sheet_name: String) -> anyhow::Result<Vec<Row>> {
    let rdr: Box<dyn io::Read> = if let Some(filename) = filename {
        Box::new(io::BufReader::new(File::open(filename)?))
    } else {
        let sheet_id = env::var("SHEET_ID")?;
        let url = format!(
            "https://docs.google.com/spreadsheets/d/{}/gviz/tq?tqx=out:csv&sheet={}",
            sheet_id, sheet_name
        );
        Box::new(reqwest::get(url).await?.bytes().await?.reader())
    };
    Reader::from_reader(rdr)
        .into_deserialize()
        .collect::<Result<_, _>>()
        .context("Cannot parse csv")
}

fn get_next_states_for_key(data: &[Row], k: &str) -> anyhow::Result<FSNextStates> {
    data.iter()
        .filter(|r| r.key.strip_prefix(k).map_or(false, |s| !s.contains('.')))
        .map(|mut row| {
            if let Some(key) = row.question.strip_prefix('#') {
                let pred = |row_: &&Row| row_.key == key;
                let map_err = || anyhow!("Didn't find {} in row {}", row.question, row.key);
                row = data.iter().find(pred).ok_or_else(map_err)?;
            };
            let key = row.key.clone() + ".";
            let next_states = get_next_states_for_key(data, &key)?;
            let map_err = || format!("Error in parsing row with key {}", row.key);
            let fs = FS::parse_row(row, next_states).with_context(map_err)?;
            Ok((row.question.clone(), fs))
        })
        .collect()
}

async fn get_finite_state(lang: Lang, filename: Option<&str>) -> anyhow::Result<FS> {
    let mut rows = get_rows(filename, lang.name()).await?;
    rows.retain(|record| !record.is_empty());
    rows.iter_mut()
        .for_each(|r| r.key = r.key.trim().to_string());
    Ok(FS::entry(lang, get_next_states_for_key(&rows, "")?))
}

pub async fn get_data(filename: Option<&str>) -> anyhow::Result<MultilangStates> {
    let load = |lang| async move { get_finite_state(lang, filename).await.map(|fs| (lang, fs)) };
    stream::iter(Lang::iter()).then(load).try_collect().await
}

#[cfg(test)]
mod test;
