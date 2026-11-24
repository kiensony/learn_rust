use task_1248::*;

#[test]
fn transfer_case() {
    assert_eq!(
        delimited_fields_rolling_load(&[9, 2, 2, 8, 0, 5, 5, 0, 8, 2, 2, 9], 3),
        vec![10u64]
    );
}
