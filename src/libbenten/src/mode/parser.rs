use serde::{ Deserialize, Deserializer };
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::methods::GenericMethodTrait;
use crate::methods::layout::{ LayoutMethod };
use crate::methods::table::TableMethod;
use crate::BentenError;

#[derive(Deserialize)]
pub struct Mode {
	#[serde(deserialize_with = "to_methods")]
	pub methods: Vec<Box<dyn GenericMethodTrait>>,
	pub bindings: Option<Vec<Binding>>,
	#[serde(deserialize_with = "get_first_method")]
	pub current_method: Box<dyn GenericMethodTrait>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Binding {
	pub function: Function,
	pub conditions: Vec<Condition>,
	pub key_codes: Vec<u16>
}

#[derive(Clone, Debug, Deserialize)]
pub enum Function {
	ChangeMethodTo(String)
}

#[derive(Debug, Deserialize)]
pub enum Condition {
	CurrentMethodIs(String),
	Empty,
	CurrentMethodIsInstanceOf(String)
}

impl Mode {
	pub fn new(id: &str, base_dir: &Path) -> Result<Self, BentenError> {
		let mode = Self::parse(&base_dir.join("modes").join(id).with_extension("mode.yaml"));

		Ok({ 
			if let Ok(mode) = mode {
				mode
			} else if let Ok(table) = TableMethod::new(id, &base_dir) {
				Mode::from(Box::new(table) as Box<dyn GenericMethodTrait>)
			} else if let Ok(layout) = LayoutMethod::new(id, &base_dir) {
				Mode::from(Box::new(layout) as Box<dyn GenericMethodTrait>)
			} else {
				mode? // If all else fails, just return the original result to be able to handle error
			}
		})
	}

	pub fn parse(path: &Path) -> Result<Self, BentenError> {
		let file = File::open(path)?;
    	let reader = BufReader::new(file);
		Ok(serde_yaml::from_reader(reader)?)
	}
}

impl From<Box<dyn GenericMethodTrait>> for Mode {
    fn from(method: Box<dyn GenericMethodTrait>) -> Self {
        Mode {
        	methods: Vec::with_capacity(0),
        	bindings: None,
        	current_method: method,
        }
    }
}

pub fn to_methods<'de, D>(deserializer: D) -> Result<Vec<Box<dyn GenericMethodTrait>>, D::Error>
where D: Deserializer<'de> {

	//TEMPORARY
	let base_dir = xdg::BaseDirectories::with_prefix("benten").unwrap().get_config_home();

    let values: Vec<String> = Vec::deserialize(deserializer)?;

    let mut out: Vec<Box<dyn GenericMethodTrait>> = Vec::new();
	for value in values {
		if let Ok(table) = TableMethod::new(&value, &base_dir) {
			out.push(Box::new(table));
		} else if let Ok(layout) = LayoutMethod::new(&value, &base_dir) {
			out.push(Box::new(layout));
		} else {
			return Err(serde::de::Error::custom(format!("Method of id: `{}` not found", value)))
		}	
	}

	Ok(out)
}

pub fn get_first_method<'de, D>(deserializer: D) -> Result<Box<dyn GenericMethodTrait>, D::Error>
where D: Deserializer<'de> {
	//TEMPORARY
	let base_dir = xdg::BaseDirectories::with_prefix("benten").unwrap().get_config_home();

 	if let Ok(value) = String::deserialize(deserializer) {
 		if let Ok(table) = TableMethod::new(&value, &base_dir) {
			return Ok(Box::new(table));
		} else if let Ok(layout) = LayoutMethod::new(&value, &base_dir) {
			return Ok(Box::new(layout));
		}	
 	}
 	
 	Err(serde::de::Error::custom("Invalid method id"))
}