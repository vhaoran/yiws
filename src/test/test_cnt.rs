#[test]
fn test_cnt_1() {
    use crate::msg_util::cnt;

    for i in 0..10000 {
        cnt::push_cnt(i, None);
    }

    println!("-------------------------");
    cnt::display_cnt();
}