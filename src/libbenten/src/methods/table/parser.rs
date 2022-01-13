use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde_with::{As, DisplayFromStr};
use std::str::FromStr;

use crate::BentenError;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
	pub id: String,
	pub encoding: String,
	#[serde(with = "As::<Vec<DisplayFromStr>>")]
	pub entries: Vec<Entry>
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Entry {
	pub character: char,
	pub sequence: String,
}

impl FromStr for Entry {
    type Err = std::num::ParseIntError;

    fn from_str(entry: &str) -> Result<Self, Self::Err> {
		let mut iter = entry.split_whitespace();

    	let character: char = iter.next().unwrap().parse::<char>().unwrap();
    	let sequence: String = iter.next().unwrap().to_string();

        Ok(Entry { character, sequence })
    }
}

impl Table {
	pub fn new(id: &str, base_dir: &Path) -> Result<Table, BentenError> {
		let path = base_dir.join("tables").join(id).with_extension("dict.yaml");
		let file = File::open(path)?;
	    let reader = BufReader::new(file);
		Ok(serde_yaml::from_reader(reader)?)
	}
}