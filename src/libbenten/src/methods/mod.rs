pub mod layout;
use layout::{ Layout, LayoutMethod, LayoutKind };

pub mod table;
use table::{ Table, TableMethod };

use crate::{ BentenResponse, BentenError };
use serde::{ Deserialize, Deserializer };
use std::collections::HashMap;
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
    pub methods: HashMap<String, Box<dyn GenericMethodTrait>>,
    pub current_method: String,
}

impl Global {
    pub fn new(id: &str, base_dir: &Path) -> Result<Self, BentenError> {
        let path = base_dir.join("layouts").join(id).with_extension("layout.zm");
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        match zmerald::de::from_reader::<_, Global>(reader) {
            Ok(mut g) => {
                g.id = id.to_string();
                return Ok(g);
            },

            // this allows the config to define a single method not within a global struct, simpler single methods
            Err(e) => {
                if let Ok(table) = TableMethod::new(id, &base_dir) {
                    return Ok(Global::from(id, Box::new(table)));
                } else {
                    match LayoutMethod::new(id, &base_dir) {
                        Ok(layout) => return Ok(Global::from(id, Box::new(layout))),
                        Err(_) => panic!("{}", e)//return Err(BentenError::ZmeraldError(e))
                    }
                }     
            }
        }
    }
}

impl Global {
    fn from(id: &str, method: Box<dyn GenericMethodTrait>) -> Self {
        let current_method = method.id().to_string();
        let mut methods = HashMap::new();
        methods.insert(current_method.clone(), method);
        Self {
            id: id.to_string(),
            methods,
            current_method: current_method,
        }
    }
}

fn from_methods<'de, D>(deserializer: D) -> Result<HashMap<String, Box<dyn GenericMethodTrait>>, D::Error>
where D: Deserializer<'de> {
    let values: Vec<Layout> = Vec::deserialize(deserializer)?;

    let mut out: HashMap<String, Box<dyn GenericMethodTrait>> = HashMap::new();
    for value in values {
        match value.kind {
            LayoutKind::Layout => { out.insert(value.id.to_string(), Box::new(LayoutMethod::from(value))); },
            LayoutKind::Table => {
                let value_name = value.id.to_string();
                if let Ok(table_method) = TableMethod::try_from(value) {
                    out.insert(value_name, Box::new(table_method));
                }
            },
            LayoutKind::Hangeul => {}
        }
    }

    Ok(out)
}