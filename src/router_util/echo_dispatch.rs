extern crate env_logger;
extern crate ws;

use crate::msg_util::rx_tx::{send_str};

// This handler simply echoes all messages back to the client
pub struct EchoDispatch {
    pub ws: ws::Sender,
}

impl ws::Handler for EchoDispatch {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        // println!("Echo handler received a message: {}", msg);
        // self.ws.send(msg)
        //prepare_rtx(cast_msg);
        send_str(msg.to_string());
        println!("---EchoDispatch->after send_str----------------------");
        Ok(())
    }
}

