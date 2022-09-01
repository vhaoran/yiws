extern crate log;

// use log::*;

use std::fmt::Display;
use std::fmt;

//extern crate toml;
extern crate serde_derive;
// extern crate serde_json;

// use serde_json::{Result, Value, Value::String};
use serde_json::Value;
use serde_derive::Deserialize;
// use std::fmt::Display;
// use std::fmt;

/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, Deserialize)]
pub struct MsgWrapper {
    pub to: Option<u64>,
    pub body: Value,
}


#[allow(unused_imports)]
#[allow(dead_code)]
impl Display for MsgWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.to.unwrap_or(0), self.body.to_string())
    }
}