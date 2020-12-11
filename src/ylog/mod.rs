extern crate log;
extern crate simplelog;
extern crate time;

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


    let path = path.join(PathBuf::from(time_file_name()));
    // path.set_file_name("sys.log");

    let s = path.to_str().unwrap().to_string();
    info!("----ylog---log file:-{}----", s);


    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Debug,
                        Config::default(),
                        TerminalMode::Mixed),
        WriteLogger::new(LevelFilter::Debug,
                         Config::default(),
                         File::create(s).unwrap()),
    ])
        .unwrap();


    info!("debug level,only for test! ");
    error!("err logger,only_only for test! ");
}

#[allow(dead_code)]
fn wd() -> Option<String> {
    // let path = env::current_dir().unwrap();
    let path = env::current_dir().unwrap();

    info!("The current directory is {}", path.display());
    Some(path.to_str().unwrap().to_string())
}

fn time_file_name() -> String {
    let t = time::OffsetDateTime::now_local();
    format!("sys_{}_{}_{}_{}_{}_{}.log", t.year(), t.month(), t.day(), t.hour(), t.hour(), t.second())
}

#[test]
fn t_fn() {
    println!(" ----{0}----", time_file_name())
}

#[test]
fn time_name() {
    println!("{}", time_file_name());
}