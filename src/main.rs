use clap::{Parser};
pub mod cmd_line_arg_parse;
pub mod func_util;
pub mod struct_util;

fn main() {
    let cli = cmd_line_arg_parse::cli::Cli::parse();
    let file_path_list = cli.get_file_path_list();
    let list_content = file_path_list.clone().join("\n");
    // std::fs::write(
    //     "./list-src.git-ignore.txt",
    //     list_content.as_bytes()
    // ).expect("msg: list-src");
    let first_path_name = file_path_list.get(15).unwrap().clone();
    if first_path_name.ends_with(".vue") {
        crate::func_util::file_parser::parse_vue(first_path_name.as_str());
    }
}
