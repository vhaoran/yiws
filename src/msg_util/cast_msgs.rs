extern crate log;

use log::*;

use crate::msg_util::cnt::{get_cnt, rm_cnt};
use ws::Sender;
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
                ws_send(ws, to, body);
            }
            _ => {
                warn!("----cast_msgs.rs---没有找到对应的uid的web socket,无法发送到客户侧-----");
            }
        }
    }

    task::spawn(send(to, body));
}

fn ws_send(ws: Sender, to: u64, body: String) {
    let r = ws.send(body.clone());

    //remove web socket cnt
    if r.is_err() {
        error!("投递到{}的消息{} 失败，可能对方不在线！", to, body);
        rm_cnt(to);
    }
}
