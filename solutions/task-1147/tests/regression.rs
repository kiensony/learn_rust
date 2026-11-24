use task_1147::*;

#[test]
fn transfer_case() {
    assert_eq!(apply_all(-1, &[&Clamp(0)]), -1);
}
