#[test]
fn test_rtx_1() {
    use crate::msg_util::rx_tx;

    rx_tx::prepare_rtx();
    let now = std::time::SystemTime::now();


    for i in 0..1000000 {
        let d = now.elapsed();
        let s = format!(" msg - {},milsec:{}", i, d.unwrap().as_millis());
        rx_tx::send_str(s);
        //std::thread::sleep(std::time::Duration::new(1, 0))
    }


    std::thread::sleep(std::time::Duration::new(40, 0));
    println!("------------ok-------------");
}
