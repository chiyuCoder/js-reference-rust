use crate::func_util::path_part;
use std::path::{PathBuf};
use path_absolutize::Absolutize;

pub struct CodeFile {
    src: String,
    depend_file_list: std::sync::Mutex<Vec<String>>,
    dir: String,
}

impl CodeFile {
    pub fn new(src: &str) -> CodeFile {
        CodeFile {
            src: String::from(src),
            dir: path_part::get_directory_from(src),
            depend_file_list: std::sync::Mutex::new(vec![]),
        }
    }

    pub fn add_depend_file(&self, file_name: &str) {
        let mut file_list = self.depend_file_list.lock().unwrap();
        let is_in_list = file_list.iter().any(|in_list| {
            return in_list.eq(file_name);
        });
        if is_in_list {
           return;
        }
        (*file_list).push(String::from(file_name));
    }

    pub fn get_depend_file_list(&self) -> Vec<String> {
        let mut file_list = self.depend_file_list.lock().unwrap();
        file_list.clone()
    }

    pub fn get_src(&self) -> String {
        return self.src.clone();
    }

    pub fn get_dir(&self) -> String {
        return self.dir.clone();
    }

    pub fn get_result_of_calc_path(&self, rel_path: &str) -> String {
        let dir = self.get_dir();
        let mut path_buf = PathBuf::from(dir.as_str());
        path_buf.push(rel_path);
        let result_path = path_buf.absolutize().unwrap();
        let result_path = result_path.to_str().unwrap();
        String::from(result_path)
    }
}
