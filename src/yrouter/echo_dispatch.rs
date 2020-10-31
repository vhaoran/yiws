extern crate env_logger;
extern crate ws;
extern crate log;

use crate::ymsg;

use log::*;


// use crate::ymsg::cnt;

// This handler simply echoes all messages back to the client
pub struct EchoDispatch {
    pub ws: ws::Sender,
}

impl ws::Handler for EchoDispatch {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        let _r = self.ws.send("welcome...".to_string());
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        debug!("Data handler received a message: {}", msg);
        match msg.as_text() {
            Ok("ping") => {
                let _r = self.ws.send("pong".to_string());
            }
            _ => { ymsg::do_dispatch(msg.to_string()); }
        }
        Ok(())
    }

    fn on_close(&mut self, _code: ws::CloseCode, _reason: &str) {
        info!(" on_close")
    }

    fn on_error(&mut self, err: ws::Error) {
        error!(" on_error: {:#?}", err)
    }

    fn on_shutdown(&mut self) {
        info!(" on_shutdown")
    }
}