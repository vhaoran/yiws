extern crate serde_json;


#[allow(unused_imports)]
#[allow(dead_code)]

//use crate::msg_util::msg_body::MsgWrapper;
use crate::msg_util::MsgWrapper;

#[test]
fn msg_body_1() {
    // use std::borrow::Borrow;
    // use serde_json::{Result, Value, Value::String};
    use serde_json::{Value};
    let data = r#"
        {
            "to": 56,
            "body": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data).unwrap();

    println!("-------------------------");
    println!("name: {}  body[0]: {}", v["to"], v["body"][0]);
    println!("body: {}", v["body"].to_string());


    let x = serde_json::to_string(&v["body"]);
    println!("-------------------------");
    println!("------------{}-------------", x.unwrap_or("no".to_string()));
}

#[test]
fn msg_body_2() {
    // use std::borrow::Borrow;
    // use serde_json::{Result, Value, Value::String};

    let data = r#"
        {
            "to": 56,
            "body": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: MsgWrapper = serde_json::from_str(data).unwrap();

    println!("-------------------------");
    println!("name: {} ", v);
    println!("name: {} ", v.body.to_string());
}
