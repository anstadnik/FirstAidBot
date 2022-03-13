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
use futures::{stream, StreamExt};
use prelude::*;

async fn get_csv(sheet_id: &str, sheet_name: &str) -> Vec<Record> {
    let url = format!(
        "https://docs.google.com/spreadsheets/d/{sheet_id}/gviz/tq?tqx=out:csv&sheet={sheet_name}",
    );
    let reader = reqwest::get(url).await.unwrap();
    let rdr = Reader::from_reader(reader.bytes().await.unwrap().reader());
    rdr.into_deserialize().map(|row| row.unwrap()).collect()
}

fn get_ordered_keys(options: &[&Record], key: Option<String>) -> Vec<String> {
    let key = key.unwrap_or_default();
    let get_index = |hierarchy: &String| hierarchy.replace(&key, "").parse().unwrap();
    let mut order: Vec<(u16, _)> = options
        .iter()
        .map(|row| (get_index(&row.hierarchy), row.option.to_owned()))
        .collect();
    order.sort();
    order.into_iter().map(|x| x.1).collect()
}

fn fill_item(data: &[Record], key: Option<String>) -> Option<FiniteStateOptions> {
    let predicate: Box<dyn Fn(&&Record) -> bool> = match &key {
        None => Box::new(|row| !row.hierarchy.contains('.')),
        Some(parent_key) => Box::new(move |row| {
            row.hierarchy.starts_with(parent_key)
                && !row.hierarchy.replacen(parent_key, "", 1).contains('.')
        }),
    };
    let options: Vec<_> = data.iter().filter(predicate).collect();
    if options.is_empty() {
        return None;
    }

    let convert_row = |row: &&Record| {
        let options = fill_item(data, Some(format!("{}.", row.hierarchy)));
        let state = FiniteState::new(row, options);
        (row.option.to_owned(), state)
    };
    let ordered_keys = get_ordered_keys(&options, key);
    let next_states = options.iter().map(convert_row).collect();
    Some(FiniteStateOptions {
        ordered_keys,
        next_states,
    })
}

async fn get_finite_state(lang: &Lang) -> FiniteState {
    FiniteState {
        link: None,
        message: lang.greet.to_string(),
        options: fill_item(&get_csv(SHEET_ID, lang.sheet).await, None),
    }
}

pub async fn get_data() -> MultilangStates {
    stream::iter(LANGS.iter())
        .then(|lang| async { (lang.name.to_string(), get_finite_state(lang).await) })
        .collect::<MultilangStatesHashMap<_, _>>()
        .await
}
