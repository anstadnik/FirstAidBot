mod data;
mod finite_state;
mod lang;

pub mod prelude {
    pub use super::data::{Cfs, CowMultLangFsExt, Data};
    pub use super::finite_state::{Fs, MultilangFs};
    pub use super::lang::Lang;
    pub use super::{get_data_from_file, get_data_from_web};
}

use self::finite_state::{Fs, Row};
use anyhow::{anyhow, Context};
use bytes::Buf;
use csv::Reader;
use finite_state::MultilangFs;
use indexmap::IndexMap;
use prelude::*;
use std::io::Read;

fn get_next_states_for_key(data: &[Row], k: &str) -> anyhow::Result<IndexMap<String, Fs>> {
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
            let fs = Fs::parse_row(row, next_states).with_context(map_err)?;
            Ok((row.question.clone(), fs))
        })
        .collect()
}

fn get_finite_state(rdr: Reader<impl Read>, lang: Lang) -> anyhow::Result<Fs> {
    let mut rows = rdr
        .into_deserialize()
        .collect::<Result<Vec<Row>, _>>()
        .context("Cannot parse csv")?;
    rows.retain(|record| !record.is_empty());
    for r in rows.iter_mut() {
        r.key = r.key.trim().to_string();
        r.question = r.question.trim().to_string();
    }
    Ok(Fs::entry(lang, get_next_states_for_key(&rows, "")?))
}

// This file is only for Ukrainian. If we will want to add more languages, it should be changed
pub fn get_data_from_file(filename: &str) -> anyhow::Result<MultilangFs> {
    // let rdr = Reader::from_reader(BufReader::new(File::open(filename)?));
    let rdr = Reader::from_path(filename)?;
    assert!(Lang::iter().count() == 1, "Only one language is supported");
    let lang = Lang::iter().next().unwrap();
    Ok([(lang, get_finite_state(rdr, lang)?)].into())
}

pub async fn get_data_from_web() -> anyhow::Result<MultilangFs> {
    let sheet_id = env!("SHEET_ID");
    assert!(Lang::iter().count() == 1, "Only one language is supported");
    let lang = Lang::iter().next().unwrap();
    let sheet_name = lang.name();
    let url = format!(
        "https://docs.google.com/spreadsheets/d/{}/gviz/tq?tqx=out:csv&sheet={}",
        sheet_id, sheet_name
    );
    let rdr = Reader::from_reader(reqwest::get(url).await?.bytes().await?.reader());
    Ok([(lang, get_finite_state(rdr, lang)?)].into())
}
