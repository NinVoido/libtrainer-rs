use std::collections::BTreeMap;
use std::path::Path;
use std::error::Error;
use std::fmt;

use serde::{Serialize, Deserialize};

use crate::error_types::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    key: String,

    #[serde(flatten)]
    values: BTreeMap<String, String>,

}

impl fmt::Display for Record {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.key)
    }
}

impl Record {

    pub fn new(key: String, values: BTreeMap<String, String>) -> Self {
        Record {
            key,
            values,
        }
    }

    pub fn copy_format(a: Record) -> Self {
        let mut res = Record {
            key: a.key.clone(),
            values: BTreeMap::new()
        };
        for i in a.values.keys() {
            if a.values.get(i) != Some(&"".to_string()) {
                res.values.insert(i.clone(), "".to_string());
            }
        }
        res
    }

    pub fn get_fields(self) -> Vec<String> {
        let mut res = Vec::new();

        for (k, v) in self.values {
            if v != "" {
                res.push(k);
            }
        }

        res
    }

    pub fn insert(&mut self, k: &String, v: String) {
        *self.values.get_mut(k).unwrap() = v;
    }

    pub fn is_full(&self) -> bool {
        for i in self.values.values() {
            if i == "" {
                return false
            }
        }
        true
    }

    // pub fn strip(&mut self) {
    //     for i in &mut self.values.keys() {
    //         if self.values[i] == "".to_string() {
    //             self.values.remove(i);
    //         }
    //     }
    // }
}


pub fn load_csv_table(path: &Path) -> Result<Vec<Record>, Box<dyn Error>>{
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_path(path)?;

    let mut result: Vec<Record> = Vec::new();

    for rec in rdr.deserialize() {
        result.push(rec?);
    }

    Ok(result)
}

pub fn diff(a: &Record, b: &Record) -> Result<BTreeMap<String, (String, String)>, DifferentKeyComp> {

    if a.key != b.key {
        return Err(DifferentKeyComp);
    }

    let mut res = BTreeMap::new();
    dbg!(a, b);
    for i in a.values.keys() {
        if b.values.contains_key(i) {
            if a.values[i].to_lowercase().trim() != b.values[i].to_lowercase().trim() {
                res.insert(i.clone(), (a.values[i].clone(), b.values[i].clone()));
            }
        }
    }

    Ok(res)
}