use task_088::*;

#[test]
fn public_examples() {
    assert_eq!(two_counters_5(4, 0), (0, 4,));
    assert_eq!(two_counters_5(13, 1), (2, 2,));
}
