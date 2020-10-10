extern crate env_logger;
extern crate ws;

pub struct ClientHandler {
    pub ws: ws::Sender,
    pub data: Vec<&'static str>,
}

impl ws::Handler for ClientHandler {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        for msg in &self.data {
            self.ws.send(*msg)?
        }
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("Data handler received a message: {}", msg);
        println!("Data handler going down.");
        match msg.as_text() {
            Ok("ping") => {
                let _r = self.ws.send("pong".to_string());
            }
            _ => {}
        }
        //self.ws.send(msg)
        Ok(())
    }
}

