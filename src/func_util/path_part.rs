use std::path::PathBuf;
use glob::glob;

pub fn get_work_dir_name() -> PathBuf {
    let current_directory = std::env::current_dir().unwrap();
    current_directory.clone()
}

pub fn get_directory_from(path_str: &str) -> String {
    let path_buf =  PathBuf::from(path_str);
    let parent_path_buf = path_buf.parent().unwrap();
    let dir_path_str = parent_path_buf.to_str().unwrap();
    return String::from(dir_path_str);
}

pub fn glob_path(path_name: &str) -> Vec<String> {
    let mut result_list = Vec::<String>::new();
    println!("path_name: {}",path_name);
    for entry in glob(path_name).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let path_name = path.to_str().unwrap();
                result_list.push(String::from(path_name));
            },
            Err(e) => println!("{:?}", e),
        }
    }
    result_list
}
