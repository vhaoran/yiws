extern crate log;

use log::*;

use crate::msg_util::cnt::get_cnt;
// use std::thread::spawn;

pub fn cast_msg(to: u64, body: String) {
    use async_std::{
        task,
        //prelude::*,
        // Future或输入输出流
    };
    async fn send(to: u64, body: String) {
        match get_cnt(to) {
            Some(ws) => {
                let _r = ws.send(body);
            }
            _ => {
                warn!("----cast_msgs.rs---没有找到对应的uid的web socket,无法发送到客户侧-----");
            }
        }
    }

    task::spawn(send(to, body));
}
