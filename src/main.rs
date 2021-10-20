use val_test::validator::validate;
use val_test::validate::Rules;
use val_test::rule_map;
use  std::collections::HashMap;
fn main() {
    let rules_hash: HashMap<&str, Vec<Rules>> = rule_map![
        "ros" => vec![Rules::In {value: "123,asd,123".to_string()}],
        "ross" => vec![Rules::GreaterThan {field:"as".to_string()}]
    ];

    let vl = serde_json::json!({ "ros": "asd", "as": 2, "ross": 3});
    println!("{:?}", validate(vl, rules_hash,  ))
}
