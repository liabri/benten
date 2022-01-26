use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::BentenError;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Table {
	pub id: String,
	pub entries: Vec<Entry>
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Entry {
	pub character: char,
	pub sequence: String,
}

impl Table {
	pub fn new(id: &str, base_dir: &Path) -> Result<Table, BentenError> {
		let path = base_dir.join("tables").join(id).with_extension("dict");
		let file = File::open(path)?;
	    let reader = BufReader::new(file);

	    let mut entries: Vec<Entry> = csv::Reader::from_reader(reader).deserialize()
	    	.map(|x| x.unwrap()).collect();

		Ok(Self {
			id: id.to_string(),
			entries,
		})
	}
}