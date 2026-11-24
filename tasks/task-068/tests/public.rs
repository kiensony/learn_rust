use task_068::*;

#[test]
fn public_examples() {
    assert_eq!(two_counters_2(4), (2, 0,));
    assert_eq!(two_counters_2(13), (6, 1,));
}
