use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub hierarchy: String,
    pub option: String,
    pub answer: String,
    pub link: Option<String>,
}

pub fn get_csv(sheet_id: &str, sheet_name: &str) -> Vec<Record> {
    let url = format!(
        "https://docs.google.com/spreadsheets/d/{sheet_id}/gviz/tq?tqx=out:csv&sheet={sheet_name}",
    );
    let reader = reqwest::blocking::get(url).unwrap();
    let rdr = Reader::from_reader(reader);
    return rdr.into_deserialize().map(|row| row.unwrap()).collect();
}
