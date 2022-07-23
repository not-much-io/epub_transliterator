use std::{collections::HashMap, fs::File, ops::AddAssign, path::PathBuf, str::Chars};

use anyhow::Result;
use log::debug;

pub struct TransliterationDictionary {
    dict: HashMap<char, char>,
    failure_report: HashMap<char, u32>,
}

impl TransliterationDictionary {
    pub fn new(path: &PathBuf) -> Result<Self> {
        let dict_file = File::open(path)?;
        Ok(TransliterationDictionary {
            dict: serde_yaml::from_reader(dict_file)?,
            failure_report: HashMap::new(),
        })
    }

    pub fn transliterate_segment(&mut self, segment: Chars) -> String {
        segment
            .map(|c| self.transliterate_char(c))
            .collect::<String>()
    }

    pub fn transliterate_char(&mut self, c: char) -> char {
        let as_lowercase = c.to_ascii_lowercase();

        if !self.dict.contains_key(&as_lowercase) {
            self.failure_report
                .entry(c)
                .or_insert(1)
                .add_assign(1);
            return c;
        }

        self.dict
            .get(&as_lowercase)
            .expect("Impossible state")
            .to_owned()
    }

    pub fn log_failure_report(&self) {
        let mut sorted_report: Vec<(char, u32)> = self
            .failure_report
            .clone()
            .into_iter()
            .collect();
        sorted_report.sort_by(|a, b| b.1.cmp(&a.1));
        debug!("Occurrences of unmapped characters:");
        for entry in sorted_report {
            debug!("{}: `{}`", entry.1, entry.0)
        }
    }
}
