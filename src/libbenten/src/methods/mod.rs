pub mod layout;
use layout::LayoutMethod;

pub mod table;
use table::TableMethod;

use crate::{ BentenResponse, BentenError };
use serde::{ Deserialize, Deserializer };
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub trait GenericMethodTrait {
    fn new(id: &str, path: &Path) -> Result<Self, BentenError> where Self: Sized;
    fn on_key_press(&mut self, key_code: u16) -> BentenResponse;
    fn on_key_release(&mut self, key_code: u16) -> BentenResponse;
    fn id(&self) -> &str;
    fn reset(&mut self);
}

#[derive(Deserialize)]
pub struct Global {
    #[serde(skip)]
    pub id: String,
    #[serde(deserialize_with = "from_id")]
    pub methods: Vec<Box<dyn GenericMethodTrait>>, // 0 = main method, the rest are part of the main method
    #[serde(skip)]
    pub current_method: usize,
}

impl Global {
    pub fn new(id: &str, base_dir: &Path) -> Result<Self, BentenError> {
        let path = base_dir.join("layouts").join(id).with_extension("layout.zm");
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        match zmerald::from_reader::<_, Global>(reader) {
            Ok(mut g) => {
                g.id = id.to_string();
                return Ok(g);
            },

            Err(_) => {
                if let Ok(table) = TableMethod::new(id, &base_dir) {
                    return Ok(Global::from(id, Box::new(table)));
                } else {
                    match LayoutMethod::new(id, &base_dir) {
                        Ok(layout) => return Ok(Global::from(id, Box::new(layout))),
                        Err(e) => return Err(e)
                    }
                }     
            }
        }
    }
}

impl Global {
    fn from(id: &str, method: Box<dyn GenericMethodTrait>) -> Self {
        let mut methods = Vec::new();
        methods.push(method);
        Self {
            id: id.to_string(),
            methods,
            current_method: 0,
        }
    }
}

fn from_id<'de, D>(deserializer: D) -> Result<Vec<Box<dyn GenericMethodTrait>>, D::Error>
where D: Deserializer<'de> {
    // TEMPORARY
    let base_dir = xdg::BaseDirectories::with_prefix("benten").unwrap().get_config_home();

    let values: Vec<String> = Vec::deserialize(deserializer)?;

    let mut out: Vec<Box<dyn GenericMethodTrait>> = Vec::new();
    for value in values {
        if let Ok(table) = TableMethod::new(&value, &base_dir) {
            out.push(Box::new(table));
        } else {
            match LayoutMethod::new(&value, &base_dir) {
                Ok(layout) => out.push(Box::new(layout)),
                Err(e) => return Err(serde::de::Error::custom(format!("could not load layout of id `{}`: {}", value, e)))
            }
        } 
    }

    Ok(out)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_parse() {
        let g = Global::new("japanese", &xdg::BaseDirectories::with_prefix("benten").unwrap().get_config_home()).unwrap();
        assert_eq!(&g.id, "japanese");
    }
}