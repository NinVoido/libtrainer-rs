use std::collections::BTreeMap;
use std::path::Path;
use std::error::Error;
use std::iter::zip;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    key: String,

    #[serde(flatten)]
    values: BTreeMap<String, String>,
}

impl Record {

    pub(crate) fn new(key: String, values: BTreeMap<String, String>) -> Self {
        return Record {
            key,
            values,
        }
    }

}


pub fn load_csv_table(path: &Path) -> Result<Vec<Record>, Box<dyn Error>>{
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_path(path)?;

    let mut result: Vec<Record> = Vec::new();

    for rec in rdr.deserialize() {
        result.push(rec?);
    }

    Ok(result)
}

#[derive(Debug, Clone, PartialEq)]
pub struct DifferentKeyComp;

pub fn diff(a: &Record, b: &Record) -> Result<BTreeMap<String, (String, String)>, DifferentKeyComp> {

    if a.key != b.key {
        return Err(DifferentKeyComp);
    }

    let mut res = BTreeMap::new();

    for (a_val, b_val) in zip(a.values.iter(), b.values.iter()) {
        if a_val.1 != b_val.1 {
            res.insert(a_val.0.to_string(), (a_val.1.to_string(), b_val.1.to_string()));
        }
    }

    Ok(res)
}