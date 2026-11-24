use task_2898::*;

#[test]
fn public_examples() {
    assert_eq!(
        flag_register_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1),
        vec![2u64, 8u64, 5u64, 0u64, 8u64]
    );
    assert_eq!(flag_register_rolling_load(&[], 0), vec![]);
}
