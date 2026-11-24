use task_1348::*;

#[test]
fn transfer_case() {
    assert_eq!(
        flag_fields_rolling_load("12,3,12,4,5 12,3,12,4,5", 3),
        vec![11u64]
    );
}
