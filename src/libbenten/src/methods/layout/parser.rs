use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::collections::{ HashMap, HashSet };

use crate::BentenError;

#[derive(Debug, Deserialize)]
pub struct Layout {
    pub id: String,
    
    pub modifiers: Vec<Modifier>,                            //will use `Name` tag to associate levels with modifiers on deserialise 
    pub levels: HashMap<u16, HashSet<ModifierIndex>>,        //<Level, Modifiers> # pointing to layout.modifiers

    pub specs: Option<HashMap<u16, Vec<Option<String>>>>,    //<KeyCode, SpecialName>
    pub keys: HashMap<u16, Vec<Option<String>>>,            //<KeyCode, Character.s>
    pub bindings: Option<HashMap<u16, Vec<Option<Function>>>>         //<KeyCode, Functions>
}

pub type ModifierIndex = usize;

#[derive(Debug, Clone, Deserialize)]
pub struct Modifier {
    pub kind: ModifierKind,
    pub key_codes: HashSet<u16>
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Deserialize)]
pub enum ModifierKind {
    Set,
    Lock,
    Latch
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum Function {
    ChangeMethodTo(String)
}

impl Layout {
    pub fn new(id: &str, base_dir: &Path) -> Result<Self, BentenError> {
        let path = base_dir.join("layouts").join(id).with_extension("layout.zm");
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(zmerald::from_reader(reader)?)
    }
}