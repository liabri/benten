use serde::{Deserialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;
use crate::BentenError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layout {
	pub id: String,
	pub encoding: String,
	pub settings: Option<Settings>,
	pub modifiers: Vec<Modifier>,
	pub special_keys: Option<HashMap<u16, Vec<String>>>,
	pub keys: Option<HashMap<u16, Vec<Option<String>>>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
	pub fill_empty_lock: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Modifier {
	pub level: u16,
	pub r#type: ModifierType,
	pub key_codes: HashSet<u16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModifierType {
	Set,
	Lock,
	Latch
}

impl Layout {
	pub fn new(id: &str, base_dir: &Path) -> Result<Layout, BentenError> {
		let path = base_dir.join("layouts").join(id).with_extension("layout.yaml");
		let file = File::open(path)?;
	    let reader = BufReader::new(file);
		Ok(serde_yaml::from_reader(reader)?)
	}
}