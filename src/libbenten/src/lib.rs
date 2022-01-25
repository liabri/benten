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
	pub fn new(config: &mut BentenConfig) -> Self {
        let dir = xdg::BaseDirectories::with_prefix("benten").unwrap().get_config_home();

        //rid id of non visible characters such as "\n"
        config.id.retain(|c| !c.is_whitespace());

		BentenEngine {
            mode: RefCell::new(Box::new(ModeHouse::new(&config.id, &dir)
                .map_err(|_| panic!("Mode `{}` not found", &config.id.replace("\n", ""))).unwrap())),
            dir
        }

        //human-panic crate
	}

    pub fn on_key_press(&mut self, key_code: u16) -> BentenResponse {
    	self.mode.borrow_mut().on_key_press(key_code)
    }

    pub fn on_key_release(&mut self, key_code: u16) -> BentenResponse {
    	self.mode.borrow_mut().on_key_release(key_code)
    }

    pub fn set_mode(&mut self, name: &str) {
    	self.mode = RefCell::new(Box::new(ModeHouse::new(name, &self.dir)
            .map_err(|_| panic!("Mode `{}` not found", &name)).unwrap()));
    }

    pub fn reset(&mut self) {
        self.mode.borrow_mut().reset();
    }
}

#[derive(Debug, PartialEq)]
pub enum BentenResponse {
    Commit(String),
    Suggest(String),
    Null, //NoChar aka KeyCode is not there
    Empty, //Was found but didnt have anything to return, intentional (such as functions like HAN key)
}

#[derive(Error, Debug)]
pub enum BentenError {
    #[error("io error `{0}`")]
    IoError(#[from] std::io::Error),
    #[error("serde_yaml error `{0}`")]
    SerdeYamlError(#[from] serde_yaml::Error),
    #[error("method not found")]
    MethodNotFound
}