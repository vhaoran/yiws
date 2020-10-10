extern crate isahc;
extern crate simplelog;

use log::*;


use std::time::Duration;
use crate::config_file::config_init::get_cfg_auth_url;

#[allow(unused_imports)]
#[allow(dead_code)]
pub fn get_uid(jwt: String) -> Option<u64> {
    use isahc::prelude::*;
    fn x(jwt: &str) -> Result<String, isahc::Error> {
        // let url = "http://127.0.0.1:9110/InnerJwt";
        let url = get_cfg_auth_url();
        let body = format!(r#"{{
          "jwt": "{}"
        }}"#, jwt);

        debug!("----yi_http.rs---{}-----", body);

        let mut response = Request::post(url)
            .header("Content-Type", "application/json")
            .timeout(Duration::from_secs(3))
            .body(body)?
            .send()?;

        // Print some basic info about the response to standard output.
        debug!("Status: {}", response.status());
        debug!("Headers: {:#?}", response.headers());

        // Read the response body as text into a string and print it.
        let s = response.text().unwrap().clone();
        debug!("text : {}", s);
        debug!("----yi_http.rs--s-{}-----", s);
        Ok(s)
    }

    let r = x(jwt.as_str());
    match r {
        Ok(s) => {
            debug!("----yi_http.rs--before parse json-{}-----", s);
            parse_json_uid(s)
        }
        _ => {
            debug!("----yi_http.rs---not fetch-----");
            Some(0)
        }
    }
}

#[allow(dead_code)]
fn parse_json_uid(str: String) -> Option<u64> {
    let l = ["{", "}", "uid", ",", ":", "\"", "\r", "\n", "\t", " "];

    let mut s = str;
    for v in l.iter() {
        s = s.replace(v, "");
    }
    debug!("--------s:----{}---------len:{}----", s, s.len());

    let r = s.parse::<u64>();
    match r {
        Ok(v) => {
            debug!("----yi_http.rs---get uid of {}-----", v);
            Some(v)
        }
        _ => None,
    }
}

#[test]
fn parse_json_1() {
    let s = r#"{
   "uid":1,
   }"#;
    let r = parse_json_uid(s.to_string());
    match r {
        Some(v) => debug!("------------{}-------------", v),
        _ => error!("------------no data-------------"),
    }
    debug!("-------------------------");
}