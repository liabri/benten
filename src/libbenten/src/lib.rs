mod methods;
mod mode;

use std::cell::RefCell;
use std::path::PathBuf;
use mode::ModeHouse;
use thiserror::Error;

pub struct BentenEngine {
    mode: RefCell<Box<ModeHouse>>,
    dir: PathBuf
}

pub struct BentenConfig {
	pub id: String
}

impl BentenEngine {
	pub fn new(config: BentenConfig) -> Self {
        let dir = xdg::BaseDirectories::with_prefix("benten").unwrap().get_config_home();
        println!("ID: {}", &config.id);
		BentenEngine {
            mode: RefCell::new(Box::new(ModeHouse::new(&config.id, &dir).unwrap())),
            dir
        }
	}

    pub fn on_key_press(&mut self, key_code: u16) -> BentenResponse {
    	self.mode.borrow_mut().on_key_press(key_code)
    }

    pub fn on_key_release(&mut self, key_code: u16) -> BentenResponse {
    	self.mode.borrow_mut().on_key_release(key_code)
    }

    pub fn current_mode(&mut self) -> String {
        self.mode.borrow_mut().id.clone()
    } 

    pub fn set_mode(&mut self, name: &str) {
    	self.mode = RefCell::new(Box::new(ModeHouse::new(name, &self.dir).unwrap()));
    }

    pub fn reset(&mut self) {
        self.mode.borrow_mut().reset();
    }
}

pub enum BentenResponse {
    Commit(String),
    Suggest(String),
    Null,
    Empty,
}

#[derive(Error, Debug)]
pub enum BentenError {
    #[error("io error `{0}`")]
    IoError(#[from] std::io::Error),
    #[error("serde_yaml error `{0}`")]
    SerdeYamlError(#[from] serde_yaml::Error),
}