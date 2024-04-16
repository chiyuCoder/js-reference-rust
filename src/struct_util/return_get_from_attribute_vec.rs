
pub enum ReturnGetFromAttribute {
    NotExists,
    ValueIsNone,
    Value(String),
}

impl ReturnGetFromAttribute {
    pub fn get_string(&self) -> String {
        match self {
            ReturnGetFromAttribute::Value(str_value) => {
                str_value.clone()
            },
            _ => String::new(),
        }
    }

    pub fn is_value_none(&self) -> bool {
        match self {
            ReturnGetFromAttribute::ValueIsNone => true,
            _ => false,
        }
    }

    pub fn is_not_exists(&self) -> bool {
        match self {
            ReturnGetFromAttribute::NotExists => true,
            _ => false,
        }
    }
}
