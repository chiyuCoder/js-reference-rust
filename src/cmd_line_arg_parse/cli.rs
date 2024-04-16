use std::path::PathBuf;
use clap::{Parser};
use path_absolutize::*;
use crate::cmd_line_arg_parse::config_util::toml_config::TomlConfig;
use crate::func_util::path_part::glob_path;
use crate::func_util::vec_string::add_if_not_in;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "DIR_PATH_NAME")]
    scope: Option<Vec<String>>,

    #[arg(short, long, value_name = "DIR_PATH_NAME")]
    config: Option<String>,
}


impl Cli {
    pub fn get_config_file(&self) -> Option<PathBuf> {
        if self.config.is_some() {
            let config_file_name = self.config.clone().unwrap();
            let path_buf = PathBuf::from(config_file_name.clone());
            if path_buf.is_absolute() {
                return Some(path_buf.clone());
            }
            let mut path_buf = crate::func_util::path_part::get_work_dir_name();
            // let result = config_file_name.split(&['/', '\\'][..]);
            //
            // result.into_iter().for_each(|part| {
            //     if !part.is_empty() {
            //         path_buf.push(part);
            //     }
            // });
            path_buf.push(config_file_name.as_str());
            let result_path_buf = path_buf.absolutize().unwrap();
            let result_path_buf = result_path_buf.to_str().unwrap();
            return Some(PathBuf::from(String::from(result_path_buf)));
        }
        return None;
    }

    pub fn get_config_in_file(&self) -> Option<TomlConfig> {
        let config_file = self.get_config_file();
        if config_file.is_some() {
            let config_path_buf = config_file.unwrap();
            let option_config = TomlConfig::read_from(&config_path_buf);
            if option_config.is_some() {
                return Some(option_config.unwrap().clone());
            }
        }
        return None;
    }

    pub fn get_scope_from_cli(&self) -> Vec<String> {
        let scope_list = self.scope.clone().unwrap_or(vec![]);
        scope_list.clone()
    }


    pub fn get_scope_list(&self) -> Vec<String> {
        let mut scope_list = self.get_scope_from_cli();
        let config_file = self.get_config_in_file();
        if config_file.is_some() {
            let extra_scope_list = config_file.unwrap().get_scope_list();
            add_if_not_in(&mut scope_list, &extra_scope_list);
        }
        return scope_list.clone();
    }

    pub fn get_file_path_list(&self) -> Vec<String> {
        let mut file_path_list = Vec::<String>::new();
        let scope_list = self.get_scope_list();
        scope_list.iter().for_each(|scope| {
            let path_list = glob_path(scope.as_str());
            add_if_not_in(&mut file_path_list, &path_list);
        });
        file_path_list
    }
}
