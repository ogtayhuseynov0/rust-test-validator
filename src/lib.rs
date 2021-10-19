//! It is validate with message, rules and ValidationResult

#[macro_export]
macro_rules! rule_map {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

pub mod validate {
    use std::collections::HashMap;

    #[derive(Eq,PartialEq,Hash)]
    pub enum Rules {
        Numeric,
        Required,
        Array,
        Between,
        Boolean,
        Date,
        Email,
        InArray,
        GreaterThan,
        LessThan,
        Equal,
        Json,
        Max,
        Min,
        Nullable,
        String
    }

    pub fn rule_messages() -> &mut HashMap<Rules,&str>{
        &mut rule_map![
        Rules::Numeric => ":field must be numeric.",
        Rules::Required => ":field is required.",
        Rules::Array => ":field must be an array.",
        Rules::Between => ":field must be between :min and :max.",
        Rules::Boolean => ":field must be boolean.",
        Rules::Date => ":field must be  date.",
        Rules::Email => ":field must be an email.",
        Rules::InArray => ":field must be in :val",
        Rules::GreaterThan => ":field must be greater than :val",
        Rules::LessThan => ":field must be less than :val",
        Rules::Equal => ":field must be equal to :val",
        Rules::Json => "Field must be an JSON.",
        Rules::Max => "Length of :field must be less than :val",
        Rules::Min => "Length of :field must be greater than :val",
        Rules::String => ":field must be string."
    ]
    }
    pub struct Message {
        pub messages: Vec<String>,
    }
    #[derive(Debug)]
    pub struct ValidationResult {
        pub(crate) message: Option<String>,
        pub(crate) field: Option<String>,
        pub(crate) valid: bool,
    }
}

pub mod validator {
    use crate::validate::*;
    use serde_json::Value;
    use std::collections::HashMap;

    pub fn validate(json: Value,  rules: HashMap<&str, Vec<Rules>>) -> ValidationResult {

        return ValidationResult {
            message: None,
            field: None,
            valid: true,
        };

    }
}
