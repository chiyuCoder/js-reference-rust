
pub fn add_if_not_in<'a>(result_list: &'a mut Vec<String>, add_list: &'a Vec<String>) -> &'a mut Vec<String> {
    add_list.iter().for_each(|path_one| {
        let is_in_list = result_list.iter().any(|in_list| {
            return in_list.eq(path_one);
        });
        if !is_in_list {
            result_list.push(path_one.clone());
        }
    });
    result_list
}
