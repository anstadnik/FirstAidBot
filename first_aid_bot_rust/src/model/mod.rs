mod finite_state;

use self::finite_state::Record;
use crate::{Lang, LANGS};
use csv::Reader;
pub use finite_state::{FiniteState, FiniteStateOptions};
use std::collections::HashMap;

pub fn get_csv(sheet_id: &str, sheet_name: &str) -> Vec<Record> {
    let url = format!(
        "https://docs.google.com/spreadsheets/d/{sheet_id}/gviz/tq?tqx=out:csv&sheet={sheet_name}",
    );
    let reader = reqwest::blocking::get(url).unwrap();
    let rdr = Reader::from_reader(reader);
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
        (
            row.option.to_owned(),
            FiniteState::new(row, fill_item(data, Some(format!("{}.", row.hierarchy)))),
        )
    };
    let ordered_keys = get_ordered_keys(&options, key);
    let next_states = options.iter().map(convert_row).collect();
    Some(FiniteStateOptions {
        ordered_keys,
        next_states,
    })
}

fn get_finite_state(sheet_id: &str, sheet_name: &str) -> FiniteState {
    let data = get_csv(sheet_id, sheet_name);
    FiniteState {
        link: None,
        message: "Що трапилось?".to_string(),
        options: fill_item(&data, None),
    }
}

pub fn get_data(sheet_id: &str) -> HashMap<Lang, FiniteState> {
    LANGS
        .iter()
        .map(|lang| (lang.clone(), get_finite_state(sheet_id, lang.sheet)))
        .collect()
}
