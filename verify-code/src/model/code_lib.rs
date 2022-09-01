use blake2::Blake2b;
use serde::{Deserialize, Serialize};
use simple_log::new;
use std::convert::TryFrom;
use std::hash;
use std::hash::Hash;
use std::io::Write;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct CodeLib {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub expired: Option<i64>,
    pub expired_str: Option<String>,
}

impl CodeLib {
    pub fn new(days: i64) -> Self {
        let exp = r_base::g::unix_sec() + days as i64 * 86400;
        let s = r_base::g::date::datetime_str_of_timestamp(exp);

        Self {
            id: Some(self::rand_id()),
            expired: Some(exp),
            expired_str: Some(s),
        }
    }
}

fn rand_id() -> String {
    let i = r_base::g::random_u64();
    hash_n(i)
}

fn hash_n(i: u64) -> String {
    let s = format!("wl_code_{i}");
    //
    use blake2::digest::{Update, VariableOutput};
    use blake2::Blake2bVar;
    // use hex_literal::hex;

    let mut hasher = Blake2bVar::new(10).unwrap();
    hasher.update(s.as_bytes());
    let mut buf = [0u8; 10];
    let r = hasher.finalize_variable(&mut buf).unwrap();
    // assert_eq!(buf, hex!("2cc55c84e416924e6400"));
    //String::from_utf8(buf.to_vec()).unwrap()
    let s = buf.iter().map(|x| x.clone()).fold("".to_string(), |a, b| {
        if a.len() == 0 {
            format!("{b:x}")
        } else {
            format!("{}{b:x}", a)
        }
    });
    s
}

#[test]
fn a_1() {
    //---------------------
    let i = 1_u64;
    let s = hash_n(i);
    println!("-----------{s:?}-----------",);

    let i = 2_u64;
    let s = hash_n(i);
    println!("-----------{s:?}-----------",);

    let i = 3_u64;
    let s = hash_n(i);
    println!("-----------{s:?}-----------",);
}
