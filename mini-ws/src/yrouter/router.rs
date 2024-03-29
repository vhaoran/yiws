extern crate env_logger;
extern crate log;
/// WebSocket server using trait objects to route
/// to an infinitely extensible number of handlers
extern crate ws;

use log::*;
use std::io::BufRead;

use crate::yrouter::client_handler::ClientHandler;
use crate::yrouter::echo_dispatch::EchoDispatch;
use crate::yrouter::url_util::{get_params_of_url_part, get_user_pwd_of_url};
use ws::{CloseCode, Request};

use crate::yauth;
use crate::ycfg;
use crate::ymsg;

const IN_TEST: bool = true;

// A WebSocket handler that routes connections to different boxed handlers by resource
pub struct Router {
    pub sender: ws::Sender,
    pub inner: Box<dyn ws::Handler>,
}

impl ws::Handler for Router {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<ws::Response> {
        // let out = self.sender.clone();
        let path = crate::yauth::decode_url(req.resource());
        debug!("-------path-----{}-------------", path);

        let host = match req.origin() {
            Ok(Some(s)) => s,
            _ => "",
        };
        debug!("---------origin---{}-------------", host);

        if path.contains("dispatch") {
            debug!("------------------\n");
            debug!("--enter dispatch-------");

            let auth = get_user_pwd_of_req(req, host);
            match auth {
                Some(pwd) => {
                    if pwd == ycfg::get_cfg_pwd() {
                        self.inner = Box::new(EchoDispatch {
                            ws: self.sender.clone(),
                        });
                        ()
                    }
                }
                _ => {
                    let _r = self.sender.close(CloseCode::Invalid);
                    ()
                }
            }

            return self.inner.on_request(req);
        }

        if path.contains("/ws") {
            debug!("------------enter /ws-------------");
            let uid = get_uid_of_req(req, path.as_str());
            debug!("--uid:{:#?}-------", uid);

            //
            match uid {
                Some(i) => {
                    self.inner = Box::new(ClientHandler {
                        ws: self.sender.clone(),
                        uid: i,
                    });

                    debug!("------------uid of {}-------------", i);

                    ymsg::push_cnt(i, Some(self.sender.clone()));
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
    if IN_TEST {
        self::get_uid_of_req_test(req)
    } else {
        self::get_uid_of_req_normal(req, path)
    }
}

fn get_uid_of_req_test(req: &ws::Request) -> Option<u64> {
    let origin = match req.origin() {
        Ok(Some(s)) => s,
        _ => "",
    };
    debug!("---------origin---{}-------------", origin);
    let path = origin;

    let l: Vec<_> = path.split("//").collect();
    if l.len() < 2 {
        return None;
    }

    let l: Vec<_> = l[1].split("@").collect();
    if l.len() < 2 {
        return None;
    }

    let id = l[0].to_string().parse::<u64>().unwrap_or(0_u64);
    Some(id)
}

fn get_uid_of_req_normal(req: &ws::Request, path: &str) -> Option<u64> {
    match get_jwt_of_req(req, path) {
        Some(jwt_str) => {
            debug!("----router.rs---jwt_str {}-----", jwt_str);
            yauth::get_uid(jwt_str.to_string())
        }
        _ => {
            warn!("----router.rs---not get jwt-----");
            None
        }
    }
}

fn get_jwt_of_req(req: &ws::Request, path: &str) -> Option<String> {
    //------------------from header---------------------
    debug!("------------get_jwt_of_req  enter-------------");
    let jwt = match req.header("Jwt") {
        Some(buf) => match std::str::from_utf8(buf) {
            Ok(v) => v,
            _ => "",
        },
        _ => "",
    };

    if jwt.len() > 0 {
        return Some(jwt.to_string());
    }

    //------------------from url---------------------
    debug!("------------get_jwt_of_req test path-------{}------", path);
    match get_params_of_url_part(path) {
        Some(m) => match m.get("jwt") {
            Some(v) => Some(v.clone().to_string()),
            _ => None,
        },
        _ => None,
    }
}

#[allow(unused_imports)]
#[allow(dead_code)]
fn get_user_pwd_of_req(req: &ws::Request, path: &str) -> Option<String> {
    debug!("------------get_jwt_of_req  enter-------------");
    let jwt = match req.header("Jwt") {
        Some(buf) => match std::str::from_utf8(buf) {
            Ok(v) => v,
            _ => "",
        },
        _ => "password",
    };

    if jwt.len() > 0 {
        return Some(jwt.to_string());
    }

    //------------------from url---------------------
    debug!("------------get_jwt_of_req test path-------{}------", path);
    match get_user_pwd_of_url(path) {
        Some((_, v)) => Some(v),
        _ => Some("password".to_string()),
    }
}

#[test]
fn get_uid_test() {
    //---------------------
    // let p = "ws://1@127.0.0.1:9999/ws";
    // let uid = self::get_uid_of_req_test(p);
    // println!("-----------{uid:?}-----------");
}
