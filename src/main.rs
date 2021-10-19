use val_test::validator::validate;
use val_test::validate::Rules;
use val_test::validate::Message;
use val_test::rule_map;
use std::collections::HashMap;

fn main() {
    let rulesHash:HashMap<&str, Vec<Rules>> = rule_map![
        "ros" => vec![Rules::Required, Rules::String]
    ];
    let vl = serde_json::json!({ "ros": true});
    println!("{:?}", validate(vl,rulesHash,  ))
}
