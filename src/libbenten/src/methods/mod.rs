pub mod layout;
pub mod table;

use crate::{ BentenResponse, BentenError };
use std::path::Path;

pub trait GenericMethodTrait {
	fn new(id: &str, path: &Path) -> Result<Self, BentenError> where Self: Sized;
    fn on_key_press(&mut self, key_code: u16) -> BentenResponse;
    fn on_key_release(&mut self, key_code: u16) -> BentenResponse;
    fn id(&self) -> &str;
    fn reset(&mut self);
}