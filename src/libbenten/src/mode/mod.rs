mod parser;
pub use parser::*;

use crate::methods::GenericMethodTrait;
use crate::methods::layout::{ LayoutMethod, LayoutHelper, LayoutMethodTrait };
use crate::{ BentenResponse, BentenError };
use std::path::Path;

pub struct ModeHouse {
	pub id: String,
	pub mode: Mode,
	pub layout: LayoutMethod
}

impl ModeHouse {
	pub fn new(id: &str, base_dir: &Path) -> Result<Self, BentenError> {
		Ok(Self {
			id: id.to_string(),
			mode: Mode::new(id, &base_dir)?,
			layout: LayoutMethod::new("mode", &base_dir)?
		})
	}

    pub fn on_key_press(&mut self, key_code: u16) -> BentenResponse {
    	let mode = &self.mode;
    	if let Some(bindings) = &mode.bindings {
    		for binding in bindings {
				let level = self.layout.calculate_level();
	            if let Some(some_key_code) = binding.key_codes.get(level) {
	                if some_key_code==&key_code && self.layout.modifiers_pressed().is_empty() {
	                	if self.are_conditions_met(&binding.conditions) {
	                		// self.execute_function(&binding.function)
	                	}

			            return BentenResponse::Null
		            }
				}
    		}
    	}

        self.mode.current_method.on_key_press(key_code)
    }
    
    pub fn on_key_release(&mut self, key_code: u16) -> BentenResponse {
    	self.mode.current_method.on_key_release(key_code)
    }

    pub fn reset(&mut self) {
    	self.mode.current_method.reset();
    }

	pub fn execute_function(&mut self, function: &Function) {
		match function {
	        Function::ChangeMethodTo(m) => {
				for method in &self.mode.methods {
					if method.id()==m {
						// self.reset();
						// self.mode.current_method = method.clone()
					}
				}                			
	    	}
	    }
	}

	// Loop every condition, if any return false it means conditions are not met // MAYBE USE `enumset` or something
	pub fn are_conditions_met(&self, conditions: &Vec<Condition>) -> bool {
		for condition in conditions {
			let boolean: bool = match &condition {
				Condition::CurrentMethodIs(c) => if c==self.mode.current_method.id() { true } else { false },
				Condition::Empty => true,//if current_method instanceof table && self.current_method.key_sequence.len()==1 { true },
				Condition::CurrentMethodIsInstanceOf(c) => true, //downcast? maybe on deserialize of the String I can assign a type there,
			};

			if !boolean {
				return false;
			}
		}

		true
	}
}