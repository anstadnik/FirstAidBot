mod finite_state;
mod load_data;
use self::load_data::{get_csv, Record};
pub use finite_state::{FiniteState, FiniteStateMsg, FiniteStateOptions};

fn fill_item(data: &Vec<Record>, key: Option<String>) -> FiniteStateOptions {
    let predicate: Box<dyn Fn(&&Record) -> bool> = match key {
        None => Box::new(|row| row.hierarchy.contains('.')),
        Some(parent_key) => Box::new(move |row| {
            row.hierarchy.starts_with(&parent_key)
                && row.hierarchy.replace(&parent_key, "").contains('.')
        }),
    };
    Some(
        data.iter()
            .filter(predicate)
            .map(|row| {
                (
                    row.option.to_owned(),
                    FiniteState::new(
                        FiniteStateMsg::new(row.link.to_owned(), row.answer.to_owned()),
                        fill_item(&data, Some(format!("{}.", row.hierarchy))),
                    ),
                )
            })
            .collect(),
    )
}

pub fn get_data(sheet_id: &str, sheet_name: &str) -> FiniteState {
    let data = get_csv(sheet_id, sheet_name);
    // data.iter().for_each(|row| println!("{:?}", row));
    FiniteState::new(
        FiniteStateMsg::new(None, "Що трапилось?".to_string()),
        fill_item(&data, None),
    )
}
