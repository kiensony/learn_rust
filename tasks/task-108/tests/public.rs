use task_108::*;

#[test]
fn public_examples() {
    assert_eq!(two_counters_10(65535, 4), (0, 4,));
    assert_eq!(two_counters_10(13, 14), (1, 3,));
}
