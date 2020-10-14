extern crate log;

use log::*;

use once_cell::sync::OnceCell;
use std::sync::mpsc::{SyncSender, sync_channel, Receiver};
use std::sync::{Arc, Mutex};
use async_std::{
    // fs::File,
    // 支持异步操作的文件结构体
    task,
    // 调用调度器
    //prelude::*,
    // Future或输入输出流
};
use crate::msg_util::cnt::display_cnt_count;

#[allow(dead_code)]
pub type DispatchCallback = fn();

#[allow(dead_code)]
pub fn prepare_rtx() -> Option<()> {
    let (tx, rx) = sync_channel::<String>(0);
    glb_tx(Some(tx));

    async fn fn_loop(rx: Receiver<String>) {
        for each in rx.iter() {
            debug!("---------received:-{}-------------", each);
            super::dispatch_msg::do_dispatch(each.clone());

            //显示连接数量
            display_cnt_count();
        }
    }

    //-----loop task-----
    task::spawn(fn_loop(rx));
    Some(())
}

#[allow(dead_code)]
pub fn send_str(s: String) -> Option<()> {
    let none: Option<SyncSender<String>> = None;
    let a = Arc::clone(glb_tx(none));
    let tx = a.lock().unwrap();

    //
    let _r = tx.send(s);
    Some(())
}

#[allow(dead_code)]
fn glb_tx(tx: Option<SyncSender<String>>) -> &'static Arc<Mutex<SyncSender<String>>> {
    static INSTANCE: OnceCell<Arc<Mutex<SyncSender<String>>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        Arc::new(Mutex::new(
            tx.unwrap().clone()
        ))
    })
}
