extern crate log;extern crate serde_json;

use std::string::String;

use log::*;
use serde_json::{Result, Value};

use crate::ymsg::cast_msgs::cast_msg;

pub fn do_dispatch(msg: std::string::String){
    debug!("----dispatch_msg.rs--receive-{}-----", msg);
    
    let r = parse_str(msg.as_str());

    // r.and_then()

    if let Some((uid, data)) = r {
            cast_msg(uid, data);
    }
}

fn parse_str(s: &str) -> Option<(u64,String)> {
    let src: Result<Value> = serde_json::from_str(s);
    let m = src.ok()?;
    let uid :u64 = m.get("to").map(|x|x.as_u64().unwrap_or(0))?;
    let data = m.get("data").map(|x|x.to_string())?;

     Some((uid,data))
}

#[allow(dead_code)]
fn parse_str_bak(s: &str) -> (Option<u64>, Option<String>) {
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

#[test]
fn a_parse_str() {
    let s = r#"
      {
        "to":1,
        "data":"234"
      }
    "#;

    let r = parse_str(s);

    println!("----dispatch_msg.rs-------{:?}-" ,r);

    println!("----dispatch_msg.rs---a----{:?}-" ,Some("abc"));
    println!("----dispatch_msg.rs---a----{:?}-" ,Some((1,"abc")));
    println!("----dispatch_msg.rs---a----{:?}-" ,"abc");
    if let Some((uid,data)) = r{
        println!("----dispatch_msg.rs---uid----{}-" ,uid);
        println!("----dispatch_msg.rs---data----{}-" ,data);
    }

}