extern crate log;

use std::sync::mpsc::{Receiver, sync_channel, SyncSender};

use async_std::{
    // fs::File,
    // 支持异步操作的文件结构体
    task,
    // 调用调度器
    //prelude::*,
    // Future或输入输出流
};
use log::*;

use crate::ymsg::cnt::display_cnt_count;
use once_cell::sync::OnceCell;
use std::borrow::Borrow;

#[allow(dead_code)]
pub type DispatchCallback = fn();

static GLB_TX: OnceCell<SyncSender<String>> = OnceCell::new();


#[allow(dead_code)]
pub fn prepare_rtx() -> Option<()> {
    let high = 10_000;
    let (tx, rx) = sync_channel::<String>(high);

    info!(" init glb_tx ....");
    set_tx(tx.clone());

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
    debug!("..enter..send_str.....");
    match GLB_TX.get() {
        Some(v) => {
            let _r = v.borrow().clone().send(s);
            debug!("send_str ...  ok finally....");
        }
        _ => {
            debug!("send_str ...  no data to access");
        }
    }

    Some(())
}


pub fn set_tx(sender: SyncSender<String>) {
    debug!("....set-tx enter.....");

    let r = GLB_TX.set(sender.clone()).unwrap();
    debug!("...after set..tx result-unwrap: {:#?}", r);
    debug!("...after set..tx dump: {:#?}", sender);
    debug!(".....glb_tx dump: {:#?}", GLB_TX.get());
}


#[allow(unused_imports)]
#[allow(dead_code)]
pub fn display_tx() {
    let src = GLB_TX.get();
    if src.is_none() {
        debug!("display ...  not data to display  dump: {:#?}", GLB_TX.get());
        return;
    }

    // let a = Arc::clone(GLB_TX.get().unwrap());
    let a = GLB_TX.get().unwrap();
    let tx = a.clone();

    //
    debug!(".....tx dump: {:#?}", tx);
}


