extern crate env_logger;
extern crate ws;
extern crate log;

use log::*;
use crate::ymsg;

// use self::ws::CloseCode;

pub struct ClientHandler {
    pub ws: ws::Sender,
    pub uid: u64,
}

impl ws::Handler for ClientHandler {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        let _r = self.ws.send("welcome...".to_string());
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        debug!("Data handler received a message: {}", msg);
        match msg.as_text() {
            Ok("ping") => {
                let s = format!("pong({})", self.uid);
                debug!("ping of {}", self.uid);
                let _r = self.ws.send(s);
                ymsg::display_cnt_count();
                // ymsg::display_tx();
                ymsg::assert_push_cnt(self.uid.clone(), Some(self.ws.clone()));
            }
            _ => {}
        }
        //self.ws.send(msg)
        Ok(())
    }

    fn on_close(&mut self, _code: ws::CloseCode, _reason: &str) {
        ymsg::rm_cnt(self.uid);
        info!(" -----------on_close: {}", self.uid);
    }

    fn on_error(&mut self, _err: ws::Error) {
        ymsg::rm_cnt(self.uid);
        error!("----------- on_error: {}", self.uid)
    }

    fn on_shutdown(&mut self) {
        ymsg::rm_cnt(self.uid);
        info!("----------- on_shutdown: {}", self.uid)
    }
}

