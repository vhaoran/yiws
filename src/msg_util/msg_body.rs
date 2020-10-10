extern crate log;

// use log::*;

use std::fmt::Display;
use std::fmt;
use super::MsgWrapper;

#[allow(unused_imports)]
#[allow(dead_code)]

impl Display for MsgWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.to.unwrap_or(0), self.body.to_string())
    }
}