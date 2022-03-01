mod finite_state;
mod load_data;

use self::{
    finite_state::parse_link,
    load_data::{get_csv, Record},
};
pub use finite_state::{FiniteState, FiniteStateOptions};

fn get_order(options: &Vec<&Record>, key: Option<String>) -> Vec<String> {
    let key = key.unwrap_or_default();
    let get_index = |hierarchy: &String| hierarchy.replace(&key, "").parse().unwrap();
    let mut order: Vec<(u16, _)> = options
        .iter()
        .map(|row| (get_index(&row.hierarchy), row.option.to_owned()))
        .collect();
    order.sort();
    order.into_iter().map(|x| x.1).collect()
}

fn fill_item(data: &Vec<Record>, key: Option<String>) -> Option<FiniteStateOptions> {
    let predicate: Box<dyn Fn(&&Record) -> bool> = match &key {
        None => Box::new(|row| !row.hierarchy.contains('.')),
        Some(parent_key) => Box::new(move |row| {
            row.hierarchy.starts_with(parent_key)
                && !row.hierarchy.replace(parent_key, "").contains('.')
        }),
    };
    let options: Vec<_> = data.iter().filter(predicate).collect();
    if options.is_empty() {
        return None;
    }
    let ordered_keys = get_order(&options, key);
    let convert_row = |row: &&Record| {
        let link = parse_link(&row.link);
        let message = row.answer.to_owned();
        let options = fill_item(&data, Some(format!("{}.", row.hierarchy)));
        let state = FiniteState {
            link,
            message,
            options,
        };
        (row.option.to_owned(), state)
    };
    let next_states = options.iter().map(convert_row).collect();
    Some(FiniteStateOptions {
        ordered_keys,
        next_states,
    })
}

pub fn get_data(sheet_id: &str, sheet_name: &str) -> FiniteState {
    let data = get_csv(sheet_id, sheet_name);
    FiniteState {
        link: None,
        message: "Що трапилось?".to_string(),
        options: fill_item(&data, None),
    }
}
