use task_1346::*;

#[test]
fn transfer_case() {
    assert_eq!(
        decimal_fields_threshold_crossing("4,9,2,2,0 4,9,2,2,0", 1),
        Some((0, 4))
    );
}
