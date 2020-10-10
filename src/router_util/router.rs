extern crate env_logger;
/// WebSocket server using trait objects to route
/// to an infinitely extensible number of handlers
extern crate ws;

use ws::CloseCode;
use crate::router_util::url_util::{get_user_pwd_of_url, get_params_of_url_part};
use crate::router_util::echo_dispatch::EchoDispatch;
use crate::router_util::client_handler::ClientHandler;
use crate::auth_util::yi_http::get_uid;
use crate::msg_util::cnt;
use crate::config_file::config_init::get_cfg_pwd;

// A WebSocket handler that routes connections to different boxed handlers by resource
pub struct Router {
    pub sender: ws::Sender,
    pub inner: Box<dyn ws::Handler>,
}

impl ws::Handler for Router {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<ws::Response> {
        // let out = self.sender.clone();
        let path = crate::auth_util::decode_url(req.resource());
        println!("-------path-----{}-------------", path);

        let host = match req.origin() {
            Ok(Some(s)) => s,
            _ => "",
        };
        println!("---------origin---{}-------------", host);

        if path.contains("dispatch") {
            let auth = get_user_pwd_of_url(host);
            match auth {
                Some((_, pwd)) => if pwd == get_cfg_pwd() {
                    self.inner = Box::new(EchoDispatch { ws: self.sender.clone() });
                    ()
                },
                _ => {
                    let _r = self.sender.close(CloseCode::Invalid);
                    ()
                }
            }
        } else if path.contains("/ws") {
            println!("------------enter /ws-------------");
            let uid = get_uid_of_req(req, path.as_str());
            match uid {
                Some(i) => {
                    self.inner = Box::new(ClientHandler {
                        ws: self.sender.clone(),
                        data: vec!["one"],
                    });

                    println!("------------uid of {}-------------", i);

                    cnt::push_cnt(i, Some(self.sender.clone()));
                }
                _ => {}
            }
        } else {
            self.inner = Box::new(NotFound {});
        }

        // default,and not matched,return 404
        self.inner.on_request(req)
    }

    // Pass through any other methods that should be delegated to the child.
    //
    // You could probably use a macro for this if you have many different
    // routers or were building some sort of routing framework.

    fn on_shutdown(&mut self) {
        self.inner.on_shutdown()
    }

    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        self.inner.on_open(shake)
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        self.inner.on_message(msg)
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        self.inner.on_close(code, reason)
    }

    fn on_error(&mut self, err: ws::Error) {
        self.inner.on_error(err);
    }
}

// This handler returns a 404 response to all handshake requests
pub struct NotFound;

impl ws::Handler for NotFound {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<ws::Response> {
        // This handler responds to all requests with a 404
        let mut res = ws::Response::from_request(req)?;
        res.set_status(404);
        res.set_reason("Not Found");
        Ok(res)
    }
}


#[allow(unused_imports)]
#[allow(dead_code)]
fn get_uid_of_req(req: &ws::Request, path: &str) -> Option<u64> {
    use crate::auth_util::yi_http;

    match get_jwt_of_req(req, path) {
        Some(jwt_str) => {
            println!("----router.rs---jwt_str {}-----", jwt_str);
            get_uid(jwt_str.to_string())
        }
        _ => {
            println!("----router.rs---not get jwt-----");
            None
        }
    }
}

fn get_jwt_of_req(req: &ws::Request, path: &str) -> Option<String> {
    //------------------from header---------------------
    println!("------------get_jwt_of_req  enter-------------");
    let jwt = match req.header("Jwt") {
        Some(buf) =>
            match std::str::from_utf8(buf) {
                Ok(v) => v,
                _ => "",
            },
        _ => "",
    };

    if jwt.len() > 0 {
        return Some(jwt.to_string());
    }

    //------------------from url---------------------
    println!("------------get_jwt_of_req test path-------{}------", path);
    match get_params_of_url_part(path) {
        Some(m) =>
            match m.get("jwt") {
                Some(v) => Some(v.clone().to_string()),
                _ => None,
            },
        _ => None,
    }
}
