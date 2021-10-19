use val_test::validator::validate;
use val_test::validate::Rules;
use val_test::validate::Message;

fn main() {
    let vl = serde_json::json!({ "ros": true});
    println!("{:?}", validate(vl,"ro",Rules::Required, Message{messages: vec!["asd".to_string()]} ))
}
