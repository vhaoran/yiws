extern crate env_logger;
extern crate log;
extern crate ws;

//extern crate log;
use log::*;

mod ymsg;

use lite_ws::yrouter::{NotFound, Router};
use lite_ws::{verify_code, ycfg, ylog};
use ws::{Builder, Settings};
/*
##  for listen to received:
  ws://<id>@127.0.0.1/ws
  e.g:  ws://1@127.0.0.1/ws
## for send msg:
  ws://root:password@127.0.0.1:9999/dispatch
  send:
  {"to":1,"data":"hello,world!"}
*/

fn main() {
    let r = verify_code::verify_code();
    if r.is_err() {
        return;
    }

    //env_logger::init();
    ylog::init_log();

    //--------读取配置文件-------------
    ycfg::init_cfg();
    ymsg::prepare_rtx();

    let cnt = format!("0.0.0.0:{}", ycfg::get_cfg_port());
    info!("------------listen at: {}-------------", cnt);

    // Listen on an address and call the closure for each connection
    if let Err(error) = Builder::new()
        .with_settings(Settings {
            max_connections: ycfg::get_cfg_ws_max() as usize,
            ..Settings::default()
        })
        .build(|out| Router {
            sender: out,
            inner: Box::new(NotFound),
        })
        .unwrap()
        .listen(cnt)
    {
        error!("Failed to create WebSocket due to {:?}", error);
    }
}
