pub mod layout;
use layout::{ Layout, LayoutMethod };

pub mod table;
use table::{ Table, TableMethod };

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
    #[serde(deserialize_with = "from_methods")]
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

fn from_methods<'de, D>(deserializer: D) -> Result<Vec<Box<dyn GenericMethodTrait>>, D::Error>
where D: Deserializer<'de> {
    let values: Vec<zmerald::value::Value> = Vec::deserialize(deserializer)?;

    let mut out: Vec<Box<dyn GenericMethodTrait>> = Vec::new();
    for value in values {
        if let Ok(table) = value.clone().into_rust::<Table>() {
            if let Ok(table_method) = TableMethod::try_from(table) {
                out.push(Box::new(table_method));
            }
        } else {
            match value.into_rust::<Layout>() {
                Ok(layout) => out.push(Box::new(LayoutMethod::from(layout))),
                Err(e) => return Err(serde::de::Error::custom(format!("error while parsing layout `{}`", e)))
            }
        } 
    }

    Ok(out)
}