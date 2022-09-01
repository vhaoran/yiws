extern crate urlencoding;

mod yi_http;

pub use self::yi_http::*;


pub fn decode_url(url: &str) -> String {
    match urlencoding::decode(url) {
        Ok(v) => v,
        _ => url.to_string(),
    }
}
