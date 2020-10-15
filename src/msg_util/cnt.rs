extern crate ws;
extern crate once_cell;
extern crate log;

use log::*;
// use std::thread;
// use std::thread::sleep;
// use std::time::Duration;
#[allow(unused_imports)]
#[allow(dead_code)]
use ws::{connect, listen, CloseCode, Handler, Message, Result, Sender, Handshake};
use std::collections::hash_map::HashMap;
use std::sync::{Arc, Mutex};

use once_cell::sync::OnceCell;
use std::borrow::Borrow;

//
#[allow(unused_imports)]
#[allow(dead_code)]
pub fn glb_cnt() -> &'static Arc<Mutex<HashMap<u64, Option<Sender>>>> {
    static INSTANCE: OnceCell<Arc<Mutex<HashMap<u64, Option<Sender>>>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let m = HashMap::new();
        Arc::new(Mutex::new(m))
    })
}


#[allow(dead_code)]
pub fn push_cnt(key: u64, sender: Option<Sender>) {
    let a = Arc::clone(glb_cnt());
    let mut m = a.lock().unwrap();

    if m.contains_key(&key) {
        let _r = m.remove(&key);
    }

    match sender {
        Some(v) => m.insert(key, Some(v.clone())),
        _ => m.insert(key, sender),
    };
}

#[allow(dead_code)]
pub fn assert_push_cnt(key: u64, sender: Option<Sender>) {
    let a = Arc::clone(glb_cnt());
    let mut m = a.lock().unwrap();

    //存在时返回，不存在时新增
    if m.contains_key(&key) {
        return;
    }

    match sender {
        Some(v) => m.insert(key, Some(v.clone())),
        _ => m.insert(key, sender),
    };
}


#[allow(dead_code)]
pub fn get_cnt(key: u64) -> Option<Sender> {
    let a = Arc::clone(glb_cnt());
    let m = a.lock().unwrap();
    let r = m.get(&key);
    match r {
        Some(v) => {
            match v {
                Some(v1) => Some(v1.borrow().clone()),
                _ => None,
            }
        }
        _ => None,
    }
}

#[allow(dead_code)]
pub fn rm_cnt(key: u64) {
    let a = Arc::clone(glb_cnt());
    let mut m = a.lock().unwrap();
    if m.contains_key(&key) {
        let _r = m.remove(&key);
    }
}

#[allow(unused_imports)]
#[allow(dead_code)]
pub fn display_cnt() {
    let a = Arc::clone(glb_cnt());
    let m = a.lock().unwrap();
    for (k, v) in m.iter() {
        let s = match v {
            Some(_) => "exist data",
            _ => "not data",
        };

        debug!("------------{}  {}-------------", k,
               s);
    }
}


#[allow(unused_imports)]
#[allow(dead_code)]
pub fn display_cnt_count() {
    let a = Arc::clone(glb_cnt());
    let m = a.lock().unwrap();

    debug!("----cnt.rs---cnt_count: {}-----", m.len());
}









