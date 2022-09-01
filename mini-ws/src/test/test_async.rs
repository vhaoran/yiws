extern crate async_std;
extern crate async_channel;


#[test]
fn async_t_01() {
    // async fn f1(ch_in: async_std::sync::Sender<String>) {
    //     loop {
    //         println!("here 1");
    //         ch_in.send("hello".into()).await;
    //         println!("here 2");
    //         std::thread::sleep(std::time::Duration::from_millis(1000));
    //     }
    // }
    //
    // async fn f2(ch_out: async_std::sync::Receiver<String>) {
    //     loop {
    //         println!("here 3");
    //         let s = ch_out.recv().await.unwrap();
    //         println!("here 4");
    //         println!("{}", s);
    //     }
    // }
    //
    // let (ch_in, ch_out) = async_std::sync::channel(1000);
    // async_std::task::block_on(async {
    //     async_std::task::spawn(f1(ch_in));
    //     f2(ch_out).await;
    // });
}