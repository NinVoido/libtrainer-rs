mod file_utils;
mod data;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::path::Path;
    use crate::data::{DifferentKeyComp, load_csv_table, diff, Record};

    #[test]
    fn load_csv() {
        let result = load_csv_table(Path::new("test1.csv")).unwrap();
        dbg!(result);
    }

    #[test]
    fn diff_err() {
        let a = Record::new("A".to_string(), BTreeMap::new());
        let b = Record::new("B".to_string(), BTreeMap::new());

        assert_eq!(diff(&a, &b), Err(DifferentKeyComp));
    }
}
