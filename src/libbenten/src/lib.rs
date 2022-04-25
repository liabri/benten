mod methods;

use std::path::PathBuf;
use methods::State;
use thiserror::Error;

pub struct BentenEngine {
    state: State,
    cfg: BentenConfig,
}

impl BentenEngine {
    pub fn new(mut cfg: BentenConfig) -> Self {
        //rid id of non visible characters such as "\n"
        cfg.id.retain(|c| !c.is_whitespace());
        let state = State::new(&cfg.id, &cfg.dir).unwrap();
            // .map_err(|_| panic!("layout `{}` not found", &cfg.id)).unwrap();

        BentenEngine { state, cfg }
    }

    pub fn on_key_press(&mut self, key_code: u16) -> BentenResponse {
        let rep = self.state.methods.get_mut(&self.state.current_method).unwrap().on_key_press(key_code);
        if let BentenResponse::Function(ref function) = rep {
            if let Some(response) = self.exec_function(function) {
                return response;
            }
        }

        return rep;
    }

    pub fn on_key_release(&mut self, key_code: u16) -> BentenResponse {
        let rep = self.state.methods.get_mut(&self.state.current_method).unwrap().on_key_release(key_code);
        if let BentenResponse::Function(ref function) = rep {
            self.exec_function(function);
        }

        return rep;
    }

    pub fn set_layout(&mut self, name: &str) {
        self.state = State::new(name, &self.cfg.dir).unwrap();
    }

    pub fn exec_function(&mut self, function: &Function) -> Option<BentenResponse> {
        match function {
            Function::ChangeMethodTo(m) => self.state.current_method = m.to_string(),
            Function::CommitThenChangeMethodTo(v, m) => {
                self.state.current_method = m.to_string();
                return Some(BentenResponse::Commit(v.to_string()));
            },            
        }  

        None
    }

    pub fn reset(&mut self) {
        for method in self.state.methods.values_mut() {
            method.reset();
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BentenResponse {
    Commit(String),
    Suggest(String),
    Undefined, //KeyCode is not defined
    Empty, //KeyCode found but didnt have anything to return, intentional (like special keys eg. Han key)
    Function(Function)
}

#[derive(Error, Debug)]
pub enum BentenError {
    #[error("`{0}`")]
    IoError(#[from] std::io::Error),
    #[error("`{0}`")]
    CsvParseError(#[from] csv::Error),
    #[error("`parsing error {0}`")]
    ZmeraldError(#[from] zmerald::de::Error),
    #[error("kb parse error")]
    KbParseError,
}

pub struct BentenConfig {
    pub id: String,
    pub dir: PathBuf
}

impl Default for BentenConfig {
    fn default() -> Self {
        BentenConfig {
            dir: xdg::BaseDirectories::with_prefix("benten").unwrap().get_config_home(),
            id: "layout id was not defined".to_string()
        }
    }
}


use serde::Deserialize;
#[derive(Debug, PartialEq, Deserialize)]
pub enum Function {
    ChangeMethodTo(String),
    CommitThenChangeMethodTo(String, String)
}