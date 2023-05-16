use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub(crate) struct Preloader {
    pub key: String,

    #[serde(flatten)]
    pub raw_data: BTreeMap<String, String>,
}