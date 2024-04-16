
#[derive(Clone)]
pub struct ResultConfig {
    scope_list: Vec<String>,
}


impl ResultConfig {
    pub fn new() -> ResultConfig {
        ResultConfig {
            scope_list: vec![],
        }
    }

    pub fn add_scope(&mut self, scope_id: &str) {
        let scope_list = self.get_scope_list();
        let is_in_scope = scope_list.iter().any(|item| {
            return item.eq(scope_id);
        });
        if is_in_scope {
            return;
        }
        self.scope_list.push(String::from(scope_id));
    }

    pub fn add_scope_by_list(&mut self, scope_id_list: &Vec<String>) {
        scope_id_list.iter().for_each(|scope_id| {
            self.add_scope(scope_id);
        });
    }

    pub fn get_scope_list(&self) -> Vec<String> {
        self.scope_list.clone()
    }
}

