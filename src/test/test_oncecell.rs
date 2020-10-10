#[allow(unused_imports)]
#[allow(dead_code)]
#[test]
fn once_1() {
    use std::sync::Arc;
    use crate::msg_util;

    let a = msg_util::cnt::glb_cnt();
    let r = Arc::clone(a);
    let mut m = r.lock().unwrap();
    m.insert(100, None);

    for i in m.keys() {
        println!("------------{}:-------------",
                 i
        );
    }

    println!("------------ok-------------");
}
