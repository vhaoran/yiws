use log::debug;
use std::{fs, io};

const FILE_PATH: &str = "./code.txt";
const ERR_MSG: &str = r#"
激活码:60d384dd194bf4d064c5: 过期时间： 2050-01-17 19:54:14
尊敬的用户：
该激活码为WS_Server_Go工程使用时间，如需购买，请前往该网址：
https://echat.vip
"#;
pub fn verify_code() -> Option<bool> {
    let mut code = std::fs::read_to_string(FILE_PATH).unwrap_or("".to_string());
    if code.len() == 0 {
        code = self::read_code();
    }
    //---verify from http-request------

    //----------------------------------
    let b = self::remote_verify(code.clone()).unwrap_or(false);
    if !b {
        debug!("{ERR_MSG}");
        println!("{ERR_MSG}");
        return None;
    }
    //
    fs::write(FILE_PATH, code.as_str()).ok()?;
    Some(true)
}

fn remote_verify(code: String) -> Option<bool> {
    let url = format!("http://echat.vip:9999/vcode?code={code}");
    let body = reqwest::blocking::get(url.as_str()).ok()?.text().ok()?;
    println!("response: {:#?}", body);
    debug!("response: {:#?}", body);
    if body.contains("true") {
        return Some(true);
    }
    None
}

fn read_code() -> String {
    println!("请输入激活码:");
    let mut input = String::new();
    let code = match io::stdin().read_line(&mut input) {
        Ok(_) => input.replace("\n", "").trim().to_string(),
        Err(e) => {
            println!("-读取激活码错误--",);
            "".to_string()
        }
    };
    //
    code
}

#[test]
fn r_a_1() {
    let s = self::read_code();
    println!("-----------{s}-----------",);
}

#[test]
fn r_1() {
    let code = "abc".to_string();
    let r = remote_verify(code);
    println!("-----------{r:?}-----------",);
}
