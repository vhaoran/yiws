extern crate toml;


use serde_derive::Deserialize;
use std::env;
use std::path::{Path};

/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub ws_port: Option<u64>,
    pub ws_send_pwd: Option<String>,
    pub auth_url: Option<Vec<String>>,

    // ws_port = 8888
    // ws_send_pwd = "password"
    // auth_url = ["http://127.0.0.1:9110/InnerJwt"]
}


pub fn read_cfg() -> Option<Config> {
    use std::fs;

    let paths = ["./", "../", "../../", "../../../", "../../../../"];
    let name = "config.toml";
    //

    let wd = wd().unwrap();
    for p in paths.iter() {
        let full = Path::new(wd.as_str()).join(p).join(name.clone());

        //full.set_file_name(name.clone());

        println!("----config_util.rs---full {}-----", full.display());
        if full.exists() {
            let text = fs::read_to_string(&full).unwrap();
            println!("-----------file text:-{}-------------", text);

            return Some(toml::from_str(text.as_str()).unwrap());
        }
    }

    //
    None
}

fn wd() -> Option<String> {
    // let path = env::current_dir().unwrap();
    let path = env::current_dir().unwrap();

    println!("The current directory is {}", path.display());
    Some(path.to_str().unwrap().to_string())
}


#[test]
fn read_cfg_1() {
    let r = read_cfg();
    if r.is_some() {
        println!("----------{:#?}---------------", r.unwrap());
    } else {
        println!("------------none -------------");
    }
}
