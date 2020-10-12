extern crate env_logger;
extern crate ws;
extern crate log;

use yi_ws::msg_util::rx_tx::prepare_rtx;
use yi_ws::router_util::router::{Router, NotFound};
use yi_ws::config_file::config_init::{init_cfg, get_cfg_port};
use yi_ws::log_util::init_log;

//extern crate log;
use log::*;

mod msg_util;

use ws::{Builder, Sender, Settings};

fn main() {
    //env_logger::init();
    init_log();

    //--------读取配置文件-------------
    init_cfg();
    prepare_rtx();

    let cnt = format!("0.0.0.0:{}", get_cfg_port());
    info!("------------listen at: {}-------------", cnt);


    // Listen on an address and call the closure for each connection
    if let Err(error) =
    ws::listen(cnt, |out| {
        Router {
            sender: out,
            inner: Box::new(NotFound),
        }
    }) {
        error!("Failed to create WebSocket due to {:?}", error);
    }

    // // Listen on an address and call the closure for each connection
    // if let Err(error) =
    // Builder::new()
    //     .with_settings(Settings {
    //         max_connections: 10_0000,
    //         ..Settings::default()
    //     })
    //     .build(|out| {
    //         Router {
    //             sender: out,
    //             inner: Box::new(NotFound),
    //         }
    //     })
    //     .unwrap()
    //     .listen(cnt) {
    //     error!("Failed to create WebSocket due to {:?}", error);
    // }
}
