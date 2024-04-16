

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TomlConfig {
    scope_list: Option<Vec<String>>,
}

impl TomlConfig {
    pub fn get_scope_list(&self) -> Vec<String> {
        self.scope_list.clone().unwrap_or(vec![])
    }

    pub fn read_from(toml_path_buf: &PathBuf) -> Option<TomlConfig> {
        let toml_path = toml_path_buf.as_path();
        if toml_path.exists() {
            let result = std::fs::read_to_string(toml_path);
            let content = result.unwrap();
            let config_result = toml::from_str::<TomlConfig>(content.as_str());
            if config_result.is_ok() {
                return Some(config_result.unwrap().clone());
            }
        }
        return None;
    }

    pub fn default() -> TomlConfig {
        TomlConfig {
            scope_list: Some(vec![]),
        }
    }
}