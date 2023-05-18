use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub(crate) enum Value {

    Number(i64),

    String(String),

}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self.clone() {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
        }
    }
}


#[derive(Serialize, Deserialize)]
pub(crate) struct Preloader {
    pub key: String,
    pub comment: Option<String>,
    #[serde(flatten)]
    pub raw_data: BTreeMap<String, Value>,
}

