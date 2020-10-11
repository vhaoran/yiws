use std::thread::{park_timeout, sleep};

#[test]
fn ws_cnt_test() {
    use async_std::{
        task,
        //prelude::*,
        // Future或输入输出流
    };

    async fn f(uid: u64) {
        println!("-------------------------");
        extern crate ws;

        use ws::{Builder, Sender, Settings};

        let url = format!("ws://0755yicai.com:8083/ws?jwt=test|{}", uid);
        use ws::{connect, CloseCode};

        connect(url, |out| {
            out.send("ping").unwrap();

            move |msg| {
                println!("-{}-----{}-",
                         std::time::UNIX_EPOCH.elapsed().unwrap().as_millis(),
                         msg);
                //std::thread::sleep(std::time::Duration::new(1, 0));
                out.close(CloseCode::Normal)
            }
        });
    }

    for i in 0..10_0000 {
        task::spawn(f(i as u64));
        std::thread::sleep(std::time::Duration::new(0, 1000_000));
    }

    loop {
        println!("------------loop---{}--", std::time::UNIX_EPOCH.elapsed().unwrap().as_millis());
        std::thread::sleep(std::time::Duration::new(1, 0))
    }
}
