extern crate env_logger;
extern crate ws;
extern crate log;


use yi_ws::{ycfg};
use yi_ws::yrouter::*;

//extern crate log;
use log::*;

mod ymsg;

use ws::{Builder, Settings};
use yi_ws::ylog;

fn main() {
    //env_logger::init();
    ylog::init_log();

    //--------读取配置文件-------------
    ycfg::init_cfg();
    ymsg::prepare_rtx();

    let cnt = format!("0.0.0.0:{}", ycfg::get_cfg_port());
    info!("------------listen at: {}-------------", cnt);


    // Listen on an address and call the closure for each connection
    // if let Err(error) =
    // ws::listen(cnt, |out| {
    //     Router {
    //         sender: out,
    //         inner: Box::new(NotFound),
    //     }
    // }) {
    //     error!("Failed to create WebSocket due to {:?}", error);
    // }


    // Listen on an address and call the closure for each connection
    if let Err(error) =
    Builder::new()
        .with_settings(Settings {
            max_connections: ycfg::get_cfg_ws_max() as usize,
            ..Settings::default()
        })
        .build(|out| {
            Router {
                sender: out,
                inner: Box::new(NotFound),
            }
        })
        .unwrap()
        .listen(cnt) {
        error!("Failed to create WebSocket due to {:?}", error);
    }
}
