use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;

use rand::seq::SliceRandom;

use crate::error_types::*;
use crate::record::{diff, load_csv_table, Record};

#[derive(Default, Clone, Debug)]
pub struct Tasks {
    format: String,
    tasks: Vec<Record>,
    last_task: Option<Record>,
}

impl Tasks {
    pub fn from_csv(file: &File) -> Result<Self, Box<dyn Error>> {
        let mut tsks = Tasks {
            tasks: load_csv_table(file)?,
            ..Default::default()
        };

        let mut reader = std::io::BufReader::new(file);
        reader.read_line(&mut tsks.format)?;

        Ok(tsks)
    }

    pub fn get_random_task(&mut self) -> &Record {
        let mut rng = rand::thread_rng();
        let task = self.tasks.choose(&mut rng).unwrap();
        self.last_task = Some(task.clone());

        return task;
    }

    pub fn get_task(&self, ind: usize) -> &Record {
        return &self.tasks[ind];
    }

    pub fn len(&self) -> usize {
        return self.tasks.len();
    }

    pub fn check_answer(
        &self,
        b: &Record,
    ) -> Result<BTreeMap<String, Vec<(String, String)>>, Box<dyn Error>> {
        return if let Some(last_task) = &self.last_task {
            Ok(diff(&last_task, &b)?)
        } else {
            Err(Box::try_from(EmptyAnswerStackCheck).unwrap())
        };
    }
}
