//! It is validate with message, rules and ValidationResult

pub mod validate {
    pub enum Rules {
        Required
    }

    pub struct Message {
        pub messages: Vec<String>
    }

    #[derive(Debug)]
    pub struct ValidationResult {
        pub(crate) message: Option<String>,
        pub(crate) field: Option<String>,
        pub(crate) valid: bool
    }
}

pub mod validator {
    use crate::validate::*;
    use serde_json::Value;

    pub fn validate(json:Value, field: &str, rule: Rules, message: Message) -> ValidationResult {
        let v = serde_json::json!(json);
        if v.is_object() {
            return if v.get(field) != None{
                ValidationResult {
                    message: None,
                    field: None,
                    valid: true
                }
            } else {
                let message: String = "Is required".to_string();
                ValidationResult {
                    message: Option::from(message),
                    field: Option::from(field.to_string()),
                    valid: false
                }
            }
        }

        return ValidationResult {
            message: None,
            field: None,
            valid: true
        }

    }
}
