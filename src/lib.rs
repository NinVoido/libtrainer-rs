mod answer;
mod error_types;
mod file_utils;
mod preloader;
pub mod record;
pub mod task;

#[cfg(test)]
mod tests {
    use crate::error_types::*;
    use crate::record::{diff, load_csv_table, Record};
    use std::collections::BTreeMap;
    use std::fs::File;

    #[test]
    fn load_csv() {
        let file = File::open("test1.csv").unwrap();
        let result = load_csv_table(&file).unwrap();;
    }

    #[test]
    fn diff_err() {
        let mut a = Record::new("A".to_string(), None, BTreeMap::new());
        let mut b = Record::new("B".to_string(), None, BTreeMap::new());

        assert_eq!(diff(&mut a, &mut b), Err(DifferentKeyComp));
    }
}
