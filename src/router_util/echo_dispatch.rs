extern crate env_logger;
extern crate ws;
extern crate log;

use log::*;

use crate::msg_util::rx_tx::{send_str};
// use crate::msg_util::cnt;

// This handler simply echoes all messages back to the client
pub struct EchoDispatch {
    pub ws: ws::Sender,
}

impl ws::Handler for EchoDispatch {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        send_str(msg.to_string());
        debug!("---EchoDispatch->after send_str----------------------");
        Ok(())
    }
}

