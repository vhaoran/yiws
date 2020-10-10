extern crate log;

use log::*;

use crate::msg_util::cnt::get_cnt;

pub fn cast_msg(to: u64, body: String) {
    debug!("------------cast msg------to {} data {}-------", to, body);
    match get_cnt(to) {
        Some(ws) => {
            let _r = ws.send(body);
        }
        _ => {
            warn!("----cast_msgs.rs---没有找到对应的uid的web socket,无法发送到客户侧-----");
        }
    }
}
