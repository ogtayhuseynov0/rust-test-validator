//! It is validate with message, rules and ValidationResult
//! Example below:
//!  let rules_hash: HashMap<&str, Vec<Rules>> = rule_map![
//!        "ros" => vec![Rules::In {value: "123,asd,123".to_string()}],
//!        "ross" => vec![Rules::GreaterThan {field:"as".to_string()}]
//!     ];
#[macro_export]
macro_rules! rule_map {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

pub mod validate {
    #[derive(Eq, PartialEq, Hash, Clone)]
    pub enum Rules {
        Numeric, //+
        Required, //+
        Array, //+
        Between { min: i64, max: i64 }, //+
        Boolean, //+
        Date,  // ----------
        Email, //+
        InArray { field: String }, //+
        GreaterThan { field: String }, //+
        LessThan { field: String }, //+
        In { value: String }, //+
        NotIn { value: String }, //+
        Equal { field: String }, // +
        EqualString { value: String }, // +
        Json, // -------------------------------------
        Max { value: i64 }, //+
        Min { value: i64 }, //+
        String, //+
        NoneExist { field: String }, // +
    }

    // ":field must be greater than :val"
    pub fn rule_messages(rule: Rules) -> String {
        match rule {
            Rules::Numeric => format!(":field must be numeric."),
            Rules::Required => format!(":field is required."),
            Rules::Array => format!(":field must be an array."),
            Rules::Between { min, max } => format!(":field must be between {}  and {}.", min, max),
            Rules::Boolean => format!(":field must be boolean."),
            Rules::Date => format!(":field must be  date."),
            Rules::Email => format!(":field must be an email."),
            Rules::InArray { field } => format!(":field must be in {}", field),
            Rules::GreaterThan { field } => format!(":field must be greater than {}", field),
            Rules::LessThan { field } => format!(":field must be less than {}", field),
            Rules::Equal { field } => format!(":field must be equal to  {}", field),
            Rules::EqualString { value } => format!(":field must be equal to  {}", value),
            Rules::Json => format!("Fields must be in an JSON."),
            Rules::Max { value } => format!("Length of :field must be less than  {}", value),
            Rules::Min { value } => format!("Length of :field must be greater than {}", value),
            Rules::String => format!(":field must be string."),
            Rules::NoneExist { field } => format!("{} not exists.", field),
            Rules::In { value } => format!(":field not exists in {}", value),
            Rules::NotIn { value } => format!(":field exists in {}", value),
        }
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

    pub fn validate(json: Value, rules: HashMap<&str, Vec<Rules>>) -> HashMap<&str, Vec<String>> {
        let mut errors: HashMap<&str, Vec<String>> = HashMap::new();

        for rule in rules.iter() {
            let res = required(rules.get(rule.0).unwrap().to_vec(), rule.0, json.clone()).1;
            if res.len()>0 {
                errors.insert(rule.0,res );
            }
        }

        return errors;
    }

    fn required(rules: Vec<Rules>, field: &str, json: Value) -> (&str, Vec<String>) {
        let mut errors: Vec<String> = Vec::new();
        for rul in rules.iter() {
            let real_value = json.get(field);
            let is_none = real_value == None;
            if is_none {
                errors.push(rule_messages(rul.clone()))
            } else {
                let res = error_checker(rul.clone(), field, json.clone());
                if !res.is_empty() {
                    errors.push(res);
                }
            }
        }
        (field, errors)
    }

    fn error_checker(rule: Rules, field: &str, json: Value) -> String {
        use regex::Regex;
        let mut msg = "".to_string();
        let real_value = json.get(field);
        match rule {
            Rules::Required => {
                if real_value.unwrap().is_null() {
                    msg = rule_messages(Rules::Required)
                }
            }
            Rules::Numeric => {
                if !real_value.unwrap().is_number() {
                    msg = rule_messages(Rules::Numeric)
                }
            }
            Rules::Array => {
                if !real_value.unwrap().is_array() {
                    msg = rule_messages(Rules::Array)
                }
            }
            Rules::Boolean => {
                if !real_value.unwrap().is_boolean() {
                    msg = rule_messages(Rules::Numeric)
                }
            }
            Rules::String => {
                if !real_value.unwrap().is_string() {
                    msg = rule_messages(Rules::String)
                }
            }
            // Rules::Json => {
            //     if !serde_json::from_str(&real_value.unwrap().to_string().as_str()).is_ok() {
            //         msg = rule_messages(Rules::Json)
            //     }
            // }
            Rules::Email => {
                let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
                let is_email = email_regex.is_match(&&*real_value.unwrap().to_string().as_str().replace('"', ""));
                if !is_email {
                    msg = rule_messages(Rules::Email)
                }
            }
            Rules::In { value } => {
                let arr = value.split(',').collect::<Vec<&str>>();
                if !arr.contains(&&*real_value.unwrap().to_string().as_str().replace('"', "")) {
                    msg = rule_messages(Rules::In { value })
                }
            }
            Rules::Between { min, max } => {
                let vl = real_value.unwrap().to_owned().as_i64();
                if vl> Option::from(max) || vl< Option::from(min) {
                    msg = rule_messages(Rules::Between { min, max })
                }
            }
            Rules::Min {value} => {
                let vl = real_value.unwrap().to_owned().to_string();
                if vl.len()< value as usize {
                    msg = rule_messages(Rules::Min { value })
                }
            }
            Rules::Max {value } => {
                let vl = real_value.unwrap().to_owned().to_string();
                if vl.len()> value as usize {
                    msg = rule_messages(Rules::Max { value })
                }
            }
            Rules::EqualString { value } => {
                if real_value.unwrap().to_string().as_str().replace('"', "") == value {
                    msg = rule_messages(Rules::EqualString {value})
                }
            }
            Rules::NotIn { value } => {
                let arr = value.split(',').collect::<Vec<&str>>();
                if arr.contains(&&*real_value.unwrap().to_string().as_str().replace('"', "")) {
                    msg = rule_messages(Rules::NotIn { value })
                }
            }
            Rules::GreaterThan { field } => {
                let second = json.get(field.clone());
                if second != None {
                    if real_value.unwrap().to_owned().as_f64() < second.unwrap().to_owned().as_f64() {
                        msg = rule_messages(Rules::GreaterThan { field })
                    }
                } else {
                    msg = rule_messages(Rules::NoneExist { field })
                }
            }
            Rules::Equal { field } => {
                let second = json.get(field.clone());
                if second != None {
                    if real_value.unwrap().to_owned().as_f64() == second.unwrap().to_owned().as_f64() {
                        msg = rule_messages(Rules::Equal { field })
                    }
                } else {
                    msg = rule_messages(Rules::NoneExist { field })
                }
            }
            Rules::LessThan { field } => {
                let second = json.get(field.clone());
                if second != None {
                    if real_value.unwrap().to_owned().as_f64() > second.unwrap().to_owned().as_f64() {
                        msg = rule_messages(Rules::LessThan { field })
                    }
                } else {
                    msg = rule_messages(Rules::NoneExist { field })
                }
            }
            _ => {
                msg = "Unknown error.".to_string()
            }
        }
        msg.replace(":field", field)
    }
}
