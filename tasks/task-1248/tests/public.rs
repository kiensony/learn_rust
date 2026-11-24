use task_1248::*;

#[test]
fn public_examples() {
    assert_eq!(
        delimited_fields_rolling_load(&[9, 2, 2, 8, 0, 5], 1),
        vec![4u64, 1u64]
    );
    assert_eq!(delimited_fields_rolling_load(&[], 0), vec![]);
}
