mod methods;
mod mode;

use std::cell::RefCell;
use std::path::PathBuf;
use mode::ModeHouse;
use methods::Global;
use thiserror::Error;

pub struct BentenEngine {
    mode: RefCell<Box<ModeHouse>>,
    cfg: BentenConfig,
}

pub struct BentenConfig {
    pub id: String,
    pub dir: PathBuf
}

impl Default for BentenConfig {
    fn default() -> Self {
        BentenConfig {
            dir: xdg::BaseDirectories::with_prefix("benten").unwrap().get_config_home(),
            id: "Layout id was not defined".to_string()
        }
    }
}

impl BentenEngine {
    pub fn new(mut cfg: BentenConfig) -> Self {
        let dir = xdg::BaseDirectories::with_prefix("benten").unwrap().get_config_home();

        //rid id of non visible characters such as "\n"
        cfg.id.retain(|c| !c.is_whitespace());

        BentenEngine {
            mode: RefCell::new(Box::new(ModeHouse::new(&cfg.id, &dir)
                .map_err(|_| panic!("Mode `{}` not found", &cfg.id.replace("\n", ""))).unwrap())),
            cfg
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
        self.mode = RefCell::new(Box::new(ModeHouse::new(name, &self.cfg.dir)
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
    #[error("csv parse error `{0}`")]
    CsvParseError(#[from] csv::Error),
    #[error("zmerald error")]
    ZmeraldError(#[from] zmerald::de::Error),
    #[error("kb parse error")]
    KbParseError,
    #[error("method not found")]
    MethodNotFound
}