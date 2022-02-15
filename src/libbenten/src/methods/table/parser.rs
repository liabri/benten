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
	pub sequence: String, //maybe try a tiny_string as this is needlessly large
}

impl Table {
	pub fn from_path(id: &str, base_dir: &Path) -> Result<Table, BentenError> {
		let path = base_dir.join("tables").join(id).with_extension("dict");
		let file = File::open(path)?;
	    let reader = BufReader::new(file);
	    let entries = csv::Reader::from_reader(reader).deserialize().collect::<Result<Vec<_>, _>>()?;

		Ok(Self {
			id: id.to_string(),
			entries,
		})
	}
}