pub mod parser;
use parser::*;

use crate::{ BentenResponse, BentenError };
use crate::methods::GenericMethodTrait;
use std::collections::HashSet;
use std::path::Path;

pub struct LayoutMethod {
    pub layout: Layout,
    pub modifiers_pressed: HashSet<u16>,
}

impl GenericMethodTrait for LayoutMethod {
    fn new(id: &str, path: &Path) -> Result<Self, BentenError> {
        let layout: Layout = Layout::new(id, &path)?;

        Ok(LayoutMethod {
            layout,
            modifiers_pressed: HashSet::new(),
        })
    }

    fn on_key_press(&mut self, key_code: u16) -> BentenResponse {
        if let Some(modifier) = self.is_get_modifier(&key_code) {
            self.on_modifier_press(&modifier, &key_code);
            return BentenResponse::Null;
        }
            
        let value = self.calculate_char(&key_code);

        if let Some(value) = value {
            BentenResponse::Commit(value)
        } else {
            BentenResponse::Null
        }
    }

    fn on_key_release(&mut self, key_code: u16) -> BentenResponse {
        if let Some(modifier) = self.is_get_modifier(&key_code) {
            self.on_modifier_release(&modifier, &key_code);
        }

        BentenResponse::Null
    }

    fn id(&self) -> &str {
        &self.layout.id
    }

    fn reset(&mut self) {
        self.modifiers_pressed.clear();
    }
}

pub trait LayoutHelper {
    fn layout(&mut self) -> &Layout;
    fn modifiers_pressed(&mut self) -> &mut HashSet<u16>;
    fn layout_n_modifiers_pressed(&mut self) -> (&Layout, &mut HashSet<u16>);
}

impl LayoutHelper for LayoutMethod {
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

pub trait LayoutMethodTrait: LayoutHelper {
    fn on_modifier_press(&mut self, modifier: &Modifier, key_code: &u16) {
        match modifier.r#type {
            ModifierType::Lock => {
                if self.modifiers_pressed().contains(key_code) {
                    self.modifiers_pressed().remove(key_code);
                } else {
                    self.modifiers_pressed().extend(&modifier.key_codes);
                }
            },

            ModifierType::Set => {
                self.modifiers_pressed().extend(&modifier.key_codes)
            },

            ModifierType::Latch => {}
        } 
    }


    fn on_modifier_release(&mut self, modifier: &Modifier, key_code: &u16) {
        match modifier.r#type {
            ModifierType::Set => {
                self.modifiers_pressed().remove(key_code);
            },

            ModifierType::Lock => {},
            ModifierType::Latch => {}
        } 
    }

    fn calculate_level(&mut self) -> usize {
        let tuple = self.layout_n_modifiers_pressed();
        for layout_modifier in &tuple.0.modifiers {
            if layout_modifier.key_codes==*tuple.1 {
                return (layout_modifier.level-1).into();
            }
        }

        return 0;
    }

    fn calculate_char(&mut self, key_code: &u16) -> Option<String> {
        let level = self.calculate_level();

        if let Some(keys) = &self.layout().keys {
            if let Some(key_code) = keys.get(key_code) {
                if let Some(character) = key_code.get(level) {
                    if let Some(c) = character {
                        return Some(c.to_owned());
                    }
                }
            }
        }

        None
    }

    fn calculate_special_key(&mut self, key_code: &u16) -> Option<String> {
        let level = self.calculate_level();

        if let Some(special_keys) = &self.layout().special_keys {
            if let Some(value_opt) = special_keys.get(key_code) {
                let opt: Option<&String> = value_opt.get(level);

                // Rely on last previous non null value, allowing special keys to be used when modifiers are pressed
                // while opt.is_none() {
                //     opt = value_opt.get(level-1);
                // }

                // return opt.map(ToOwned::to_owned);
                if let Some(value) = opt {
                    return Some(value.to_owned());
                }
            }
        }

        None    
    }

    fn is_get_modifier(&mut self, key_code: &u16) -> Option<Modifier> {
        for modifier in &self.layout().modifiers {
            if modifier.key_codes.contains(key_code) {
                return Some(modifier.clone())
            }
        }

        None
    }

    fn is(&mut self, key_code: &u16, name: &str) -> bool {
        if let Some(key) = self.calculate_special_key(key_code) {
            if key==name {
                return true;
            }
        }

        false
    }
}

impl LayoutMethodTrait for LayoutMethod {}