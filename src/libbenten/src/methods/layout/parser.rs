use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::collections::{ HashMap, HashSet };

use crate::{ Function, BentenError };

#[derive(Deserialize)]
pub struct Layout {
    pub id: String,
    pub kind: LayoutKind,
    
    pub modifiers: Vec<Modifier>,                            //will use `Name` tag to associate levels with modifiers on deserialise 
    pub levels: HashMap<u16, HashSet<ModifierIndex>>,        //<Level, Modifiers> # pointing to layout.modifiers

    pub specs: Option<HashMap<u16, Vec<Option<String>>>>,    //<KeyCode, SpecialName>
    pub keys: HashMap<u16, Vec<Option<String>>>,            //<KeyCode, Character.s>
    pub bindings: Option<HashMap<u16, Vec<Option<Function>>>>         //<KeyCode, Functions>
}

#[derive(Deserialize)]
pub enum LayoutKind {
    Layout,
    Table,
    Hangeul,
}

pub type ModifierIndex = usize;

/* ???
pub type KeyCodes = HashSet<u16>;
pub enum Modifier {
    Set(KeyCodes),
    Lock(KeyCodes),
    Latch(KeyCodes),
}
??? */

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

impl Layout {
    pub fn from_path(id: &str, base_dir: &Path) -> Result<Self, BentenError> {
        let path = base_dir.join("layouts").join(id).with_extension("layout.zm");
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(zmerald::from_reader(reader).unwrap())
    }
}