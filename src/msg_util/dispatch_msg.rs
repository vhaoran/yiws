extern crate serde_json;

use serde_json::{Result, Value};
use crate::msg_util::cast_msgs::cast_msg;
use std::string::String;

pub fn do_dispatch(msg: std::string::String) {
    println!("----dispatch_msg.rs--receive-{}-----", msg);
    
    let (uid, data) = parse_str(msg.as_str());

    match (uid, data) {
        (Some(u), Some(s)) => {
            cast_msg(u, s);
        }
        _ => {}
    }
}

fn parse_str(s: &str) -> (Option<u64>, Option<String>) {
    let src: Result<Value> = serde_json::from_str(s);
    match src {
        Ok(m) =>
            (match m.get("to") {
                Some(i) => i.as_u64(),
                _ => Some(0),
            },
             match m.get("data") {
                 Some(i) => Some(i.to_string()),
                 _ => Some("".to_string()),
             }
            ),
        _ => (Some(0), Some("".to_string()))
    }
}