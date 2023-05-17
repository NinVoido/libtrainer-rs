use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::iter::zip;

use crate::error_types::*;
use crate::preloader::Preloader;

#[derive(Debug, Clone)]
pub struct Record {
    key: String,
    comment: Option<String>,
    pub(crate) values: BTreeMap<String, Vec<String>>,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.key)
    }
}

impl From<Preloader> for Record {
    fn from(pre: Preloader) -> Self {
        let mut result = Record::new(pre.key, pre.comment, BTreeMap::new());
        for (k, v) in pre.raw_data.iter() {
            let mut splitted: Vec<String> = Vec::new();
            for i in v.split("#") {
                if i == "" {
                    continue;
                } else {
                    splitted.push(i.to_string());
                }
            }
            splitted.sort();
            if splitted.len() != 0 {
                result.values.insert(k.to_string(), splitted);
            }
        }
        result
    }
}
impl Record {
    pub fn new(
        key: String,
        comment: Option<String>,
        values: BTreeMap<String, Vec<String>>,
    ) -> Self {
        Record {
            key,
            comment,
            values,
        }
    }

    pub fn copy_format(a: Record) -> Self {
        let mut res = Record {
            key: a.key.clone(),
            comment: a.comment,
            values: BTreeMap::new(),
        };
        for i in a.values.keys() {
            if a.values.get(i) != Some(&vec!["".to_string()]) {
                // format should be same length as original
                res.values.insert(i.clone(), vec!["".to_string()]);
            }
        }
        res
    }

    pub fn get_fields(self) -> Vec<String> {
        self.values.keys().map(|s| s.clone()).collect()
    }

    pub fn field_len(self, k: &String) -> usize {
        self.values[k].len()
    }

    pub fn replace(&mut self, k: &String, v: Vec<String>) {
        *self.values.get_mut(k).unwrap() = v;
    }

    pub fn comment(self) -> Option<String> {
        self.comment
    }
    pub fn is_full(&self) -> bool {
        for i in self.values.values() {
            if i.len() == 0 {
                return false;
            }
        }
        true
    }
}

pub fn load_csv_table(file: &File) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(BufReader::new(file));

    let mut result: Vec<Record> = Vec::new();

    for rec in rdr.deserialize::<Preloader>() {
        result.push(Record::from(rec.unwrap()))
    }

    Ok(result)
}

pub fn diff(
    a: &Record,
    b: &Record,
) -> Result<BTreeMap<String, Vec<(String, String)>>, DifferentKeyComp> {
    if a.key != b.key {
        return Err(DifferentKeyComp);
    }

    let mut res: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();

    for i in a.values.keys() {
        for (first, second) in zip(&a.values[i], &b.values[i]) {
            if first.to_lowercase().trim() != second.to_lowercase().trim() {
                if res.contains_key(i) {
                    res.get_mut(i)
                        .map(|val| val.push((first.clone(), second.clone())));
                } else {
                    res.insert(i.clone(), vec![(first.clone(), second.clone())]);
                }
            }
        }
    }

    Ok(res)
}
