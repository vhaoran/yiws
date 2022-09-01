#[test]
fn test_cnt_1() {
    use crate::ymsg;

    for i in 0..10000 {
        ymsg::push_cnt(i, None);
    }

    println!("-------------------------");
    ymsg::display_cnt();
}