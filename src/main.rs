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

fn main() {
    //env_logger::init();
    init_log();

    //--------读取配置文件-------------
    init_cfg();
    prepare_rtx();

    let cnt = format!("0.0.0.0:{}", get_cfg_port());
    info!("------------listen at: {}-------------", cnt);


    // Listen on an address and call the closure for each connection
    if let Err(error) = ws::listen(cnt, |out| {
        // Use our router as the handler to route the new connection
        Router {
            sender: out,
            // Default to returning a 404 when the route doesn't match.
            // You could default to any handler here.
            inner: Box::new(NotFound),
        }
    }) {
        // Inform the user of failure
        error!("Failed to create WebSocket due to {:?}", error);
    }
}
