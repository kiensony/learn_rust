use task_1348::*;

#[test]
fn public_examples() {
    assert_eq!(flag_fields_rolling_load("12,3,12,4,5", 1), vec![3u64, 5u64]);
    assert_eq!(flag_fields_rolling_load("", 0), vec![]);
}
