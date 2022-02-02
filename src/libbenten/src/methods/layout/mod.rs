pub mod parser;
use parser::*;

use crate::{ BentenResponse, BentenError };
use crate::methods::GenericMethodTrait;
use std::collections::HashSet;
use std::path::Path;
use itertools::Itertools;
use std::iter::FromIterator;

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
        match modifier.kind {
            ModifierKind::Lock => {
                if self.modifiers_pressed().contains(key_code) {
                    self.modifiers_pressed().remove(key_code);
                } else {
                    self.modifiers_pressed().extend(&modifier.key_codes);
                }
            },

            ModifierKind::Set => {
                self.modifiers_pressed().extend(&modifier.key_codes)
            },

            // remove from modifiers_pressed on ANY key press
            ModifierKind::Latch => {}
        } 
    }


    fn on_modifier_release(&mut self, modifier: &Modifier, key_code: &u16) {
        match modifier.kind {
            ModifierKind::Set => {
                self.modifiers_pressed().remove(key_code);
            },

            _ => {}
        } 
    }

    fn calculate_level(&mut self) -> Option<usize> {
        let (layout, modifiers_pressed) = self.layout_n_modifiers_pressed();

        if modifiers_pressed.is_empty() {
            return Some(0);
        }

        for (level, modifier_indexes) in &layout.levels {
            // if a modifier contains more than 1 keycode they are defined with an OR relationship, 
            // therefore we shall split it into different modifiers when comparing, making it a 
            // cartesian product.

            //convert HashSet<ModifierIndex> -> Vec<HashSet<u16>> where u16: KeyCode
            //optimisation: move this to the deserialisation phase
            let modifiers: Vec<HashSet<u16>> = modifier_indexes.iter().map(|i| 
                layout.modifiers[*i].key_codes.iter().copied()
            ).multi_cartesian_product().map(HashSet::from_iter).collect();

            for modifier in modifiers {
                if *modifiers_pressed==modifier {
                    return Some((level-1).into());
                }
            }
        }

        None
    }

    fn calculate_char(&mut self, key_code: &u16) -> Option<String> {
        if let Some(level) = self.calculate_level() {
            if let Some(key_code) = self.layout().keys.get(key_code) {
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
        if let Some(level) = self.calculate_level() {
            if let Some(special_keys) = &self.layout().specs {
                if let Some(value_opt) = special_keys.get(key_code) {
                    if let Some(value) = value_opt.get(level) {
                        if let Some(v) = value {
                            // Rely on last previous non null value, allowing special keys to be used when modifiers are pressed
                            // while opt.is_none() {
                            //     opt = value_opt.get(level-1);
                            // }

                            return Some(v.to_owned());
                        }
                    }
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