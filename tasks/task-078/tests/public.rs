use task_078::*;

#[test]
fn public_examples() {
    assert_eq!(two_counters_3(4, 0), (1, 1,));
    assert_eq!(two_counters_3(13, 3), (5, 1,));
}
