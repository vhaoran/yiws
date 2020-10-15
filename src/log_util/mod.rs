extern crate log;
extern crate simplelog;

use log::*;
use simplelog::*;

use std::fs::File;
use std::env;
use std::path::{PathBuf};

pub fn init_log() {
    let path: PathBuf = env::current_dir().unwrap()
        .join("logs");
    if !path.exists() {
        let _r = std::fs::create_dir(path.clone());
    }


    let path = path.join(PathBuf::from("sys.log"));
    // path.set_file_name("sys.log");

    let s = path.to_str().unwrap().to_string();
    println!("----mod.rs---full-{}----", s);

    CombinedLogger
    ::init(vec![
        TermLogger::new(LevelFilter::Debug,
                        Config::default(),
                        TerminalMode::Mixed),
        WriteLogger::new(LevelFilter::Debug,
                         Config::default(),
                         File::create(s).unwrap()),
    ])
        .unwrap();


    debug!("debug level,only for test! ");
    error!("err logger,only for test! ");
}

#[allow(dead_code)]
fn wd() -> Option<String> {
    // let path = env::current_dir().unwrap();
    let path = env::current_dir().unwrap();

    println!("The current directory is {}", path.display());
    Some(path.to_str().unwrap().to_string())
}

