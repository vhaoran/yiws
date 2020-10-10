pub mod msg_body;
pub mod cnt;
pub mod rx_tx;
pub mod cast_msgs;
pub mod dispatch_msg;

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
