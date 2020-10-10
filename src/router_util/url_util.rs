extern crate urlencoding;
extern crate log;

// use log::*;

use std::collections::HashMap;

pub fn get_user_pwd_of_url(url: &str) -> Option<(String, String)> {
    let l: Vec<_> = url.split("//").collect();
    if l.len() < 2 {
        return None;
    }

    let l: Vec<_> = l[1].split("@").collect();
    if l.len() < 2 {
        return None;
    }
    let l: Vec<_> = l[0].split(":").collect();
    if l.len() < 2 {
        return None;
    }

    Some((l[0].to_string(), l[1].to_string()))
}

//
pub fn get_prefix_of_url_part(url: &str) -> Option<String> {
    let url_ok = match urlencoding::decode(url) {
        Ok(v) => v,
        _ => url.to_string(),
    };

    let l: Vec<_> = url_ok.split("?").collect();
    if l.len() < 2 {
        return Some(url.to_string());
    }
    Some(l[0].to_string())
}

pub fn get_params_of_url_part(url: &str) -> Option<HashMap<String, String>> {
    let url_ok = match urlencoding::decode(url) {
        Ok(v) => v,
        _ => url.to_string(),
    };

    let l: Vec<_> = url_ok.split("?").collect();
    if l.len() < 2 {
        return None;
    }

    let ll: Vec<_> = l[1].split("&").collect();

    let mut m: HashMap<String, String> = HashMap::new();
    let mut b = false;
    for i in ll.iter() {
        let l2: Vec<_> = i.split("=").collect();
        if l2.len() == 2 {
            m.insert(l2.get(0).unwrap().to_string(),
                     l2.get(1).unwrap().to_string(),
            );
            b = true;
        }
    }

    match b {
        true => Some(m),
        _ => None,
    }
}
