pub mod parser;
pub use parser::*;

use std::collections::HashSet;
use crate::methods::GenericMethodTrait;
use crate::methods::layout::{ LayoutMethodTrait, LayoutHelper, parser::* };
use crate::BentenResponse;
use std::path::Path;
use crate::BentenError;

pub struct TableMethod {
	/// Layout variables
	pub layout: Layout,
	pub modifiers_pressed: HashSet<u16>, //maybe convert these two into a `LayoutMethod`

	/// Table variables
	pub table: Table,
	pub relative_entries: Vec<Entry>,
	pub key_sequence: String,
	pub index: usize
}

impl TryFrom<Table> for TableMethod {
    type Error = BentenError;

    fn try_from(table: Table) -> Result<Self, Self::Error> {
        //temporary
        let path = xdg::BaseDirectories::with_prefix("benten").unwrap().get_config_home();

        Ok(TableMethod {
            layout: Layout::from_path(&table.id, &path)?,
            table,
            modifiers_pressed: HashSet::new(),
            relative_entries: Vec::new(),
            key_sequence: String::with_capacity(5),
            index: 0
        })
    }  
}

//feature: copy previous character key bind, kinda like a repition mark, will need a var "previous character" buf in TableMethod
impl GenericMethodTrait for TableMethod {
    fn new(id: &str, path: &Path) -> Result<Self, BentenError> {
        Ok(TableMethod {
            table: Table::from_path(id, &path)?,
            layout: Layout::from_path(id, &path)?,
            modifiers_pressed: HashSet::new(),
            relative_entries: Vec::new(),
            key_sequence: String::with_capacity(5),
            index: 0
        })
    }

    fn on_key_press(&mut self, key_code: u16) -> BentenResponse {
        if let Some(modifier) = self.is_get_modifier(&key_code) {
            self.on_modifier_press(&modifier, &key_code);
            return BentenResponse::Null;
        }

        let mut commit = false;
    	match self.calculate_special_key(&key_code).as_deref() {
    		Some("commit") => commit = true,
    		Some("backspace") => { self.key_sequence.pop(); },
    		Some("next") => self.index = self.index+1,
    		Some("prev") => self.index = self.index-1,
    		_ => {},
    	}

    	if let Some(c) = self.calculate_char(&key_code) {
    		self.key_sequence.push_str(&c);
    	}

        if let Some(value) = self.calculate_char_dict() {
            if commit {
                return BentenResponse::Commit(value)
            } else {
                return BentenResponse::Suggest(value)
            }
        } else {
            return BentenResponse::Null
        }
    }

    fn on_key_release(&mut self, key_code: u16) -> BentenResponse {
        if let Some(modifier) = self.is_get_modifier(&key_code) {
            self.on_modifier_release(&modifier, &key_code);
        }

        return BentenResponse::Null
    }

    fn id(&self) -> &str {
        &self.layout.id
    }

    fn reset(&mut self) {
        self.index = 0;
        self.relative_entries.clear();
        self.key_sequence.clear();
        self.modifiers_pressed.clear();
    }
}

impl LayoutHelper for TableMethod {
    fn layout(&mut self) -> &Layout {
        &self.layout
    }

    fn modifiers_pressed(&mut self) -> &mut HashSet<u16> {
        &mut self.modifiers_pressed
    }
        
    fn layout_n_modifiers_pressed(&mut self) -> (&Layout, &mut HashSet<u16>) {
        (&self.layout, &mut self.modifiers_pressed)
    }
}

impl LayoutMethodTrait for TableMethod {}

impl TableMethod {
    pub fn calculate_char_dict(&mut self) -> Option<String> {
        //Tolerate index
        if self.index>=self.relative_entries.len() {
            self.index = 0
        }

        // let key_sequence: &str = &self.key_sequence;

        //If relative entries is not yet made, make it
        // if key_sequence.len()==1 { 
        self.relative_entries.clear();
            for entry in &self.table.entries {
                if entry.sequence.starts_with(&self.key_sequence) {
                    self.relative_entries.push(entry.clone());
                }
            }
  //    } else {
  //        //Filter by remove non-matching entries from relative_entries
  //        self.relative_entries.retain(|entry| entry.sequence.starts_with(&*key_sequence));
        // }

        //Get candidate
        if let Some(entry) = self.relative_entries.get(self.index).map(|x| x.to_owned()) {
            return Some(entry.character.to_string());
        }

        None
    }
}