// impl Cnt {}
extern crate log;

use log::*;

use once_cell::sync::OnceCell;
use async_std::sync::Arc;
use std::sync::Mutex;
use super::config_util::Config;
use crate::ycfg::config_util::read_cfg;

pub fn init_cfg() {
    let r = glb_cfg();
    debug!("------------读取config.toml配置完成--------{:#?}-----", r);
}

pub fn get_cfg() -> Option<Config> {
    let a = Arc::clone(glb_cfg());
    let c = a.lock().unwrap();

    Some(Config {
        ws_port: c.ws_port.clone(),
        ws_send_pwd: c.ws_send_pwd.clone(),
        auth_url: c.auth_url.clone(),
        ws_max: c.ws_max,
    })
}

pub fn get_cfg_pwd() -> String {
    match get_cfg() {
        Some(c) => c.ws_send_pwd.unwrap(),
        _ => "password".to_string(),
    }
}


pub fn get_cfg_port() -> u64 {
    match get_cfg() {
        Some(c) => c.ws_port.unwrap(),
        _ => 9999,
    }
}

pub fn get_cfg_ws_max() -> u64 {
    match get_cfg() {
        Some(c) => c.ws_max.unwrap(),
        _ => 10000,
    }
}

pub fn get_cfg_auth_url() -> String {
    match get_cfg() {
        Some(c) => {
            let l = c.auth_url.unwrap();
            match l.len() {
                0 => "".to_string(),
                1 => l[0].to_string(),
                count => {
                    let i = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs() % (count as u64);
                    l[i as usize].to_string()
                }
            }
        }
        _ => "".to_string(),
    }
}


//
fn glb_cfg() -> &'static Arc<Mutex<Config>> {
    static INSTANCE: OnceCell<Arc<Mutex<Config>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let cfg = read_cfg();
        match cfg {
            Some(c) => {
                Arc::new(Mutex::new(c))
            }
            _ => panic!("配置文件读取失败config.toml"),
        }
    })
}


#[test]
fn cfg_init_1() {
    init_cfg();
    println!("------------ok-------------");
}

#[test]
fn get_cfg_1() {
    init_cfg();
    let x = get_cfg();

    println!("------------ok----{:#?}---------", x.unwrap());
    println!("--pwd:----------{}-------------", get_cfg_pwd());
}

#[test]
fn get_url_cfg_1() {
    init_cfg();

    for _i in 0..100 {
        let x = get_cfg_auth_url();
        println!("--pwd:----------{}-------------", x);
        std::thread::sleep(std::time::Duration::new(1, 0));
    }
}



