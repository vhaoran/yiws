#[test]
fn test_rtx_1() {
    use crate::ymsg;

    ymsg::prepare_rtx();
    let now = std::time::SystemTime::now();


    for i in 0..1000000 {
        let d = now.elapsed();
        let s = format!(" msg - {},milsec:{}", i, d.unwrap().as_millis());
        ymsg::send_str(s);
        //std::thread::sleep(std::time::Duration::new(1, 0))
    }


    std::thread::sleep(std::time::Duration::new(40, 0));
    println!("------------ok-------------");
}
