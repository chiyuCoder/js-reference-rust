use crate::struct_util::code_file::CodeFile;
use crate::struct_util::return_get_from_attribute_vec::ReturnGetFromAttribute;
use deno_ast::parse_module;
use deno_ast::MediaType;
use deno_ast::ParseParams;
use deno_ast::SourceTextInfo;
use log::debug;

fn get_from_attribute_vec(attribute_vec: &Vec<vue_sfc::ast::Attribute>, target_name: &str) -> ReturnGetFromAttribute {
    let target = attribute_vec.iter().find(|one| {
        let attr = one.clone().clone();
        let attr_name = attr.0.as_str();
        return attr_name.eq(target_name);
    });
    if target.is_none() {
        return ReturnGetFromAttribute::NotExists;
    }
    let target_attribute = target.unwrap().clone();
    let target_value = target_attribute.1;
    if target_value.is_none() {
        return ReturnGetFromAttribute::ValueIsNone;
    }
    let attr_value = target_value.unwrap();
    let attr_value = attr_value.as_str();
    return ReturnGetFromAttribute::Value(String::from(attr_value));
}

pub fn parse_vue(path_like: &str) {
    let vue_content = std::fs::read_to_string(
        path_like
    ).unwrap();
    let section_list = vue_sfc::parse(vue_content.as_str()).expect("parse error");
    let code_file = CodeFile::new(path_like);
    section_list.iter().for_each(|section| {
       match section {
            vue_sfc::Section::Block(block) => {
                let block_name = block.name.as_str();
                if block_name.eq("script") {
                    let src_depend = get_from_attribute_vec(&block.attributes, "src");
                    let src_depend = src_depend.get_string();
                    if src_depend.is_empty() {
                        let content = block.content.to_string();
                        parse_typescript(
                            code_file.get_src().as_str(),
                            content.as_str()
                        );
                    } else {
                        let src_file = code_file.get_result_of_calc_path(src_depend.as_str());
                        code_file.add_depend_file(src_file.as_str());
                    }
                    //
                }
            },
           _ => {},
       }
    });
    println!("{:?}", code_file.get_depend_file_list());
}

pub fn parse_typescript(
    file_name: &str,
    source_text: &str
) {
    let text_info = SourceTextInfo::new(source_text.into());
    let parse_url = format!("file://{}", file_name);
    let parsed_source = parse_module(ParseParams {
        specifier: deno_ast::ModuleSpecifier::parse(parse_url.as_str()).unwrap(),
        media_type: MediaType::TypeScript,
        text_info,
        capture_tokens: true,
        maybe_syntax: None,
        scope_analysis: false,
    }).expect("should parse");
    let ast = parsed_source.module();
    println!("abc--run:{:?}", ast);
}
