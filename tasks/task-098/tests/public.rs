use task_098::*;

#[test]
fn public_examples() {
    assert_eq!(two_counters_8(4, 0), (0, 4,));
    assert_eq!(two_counters_8(13, 1), (1, 4,));
}
