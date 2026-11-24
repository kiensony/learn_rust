use task_849::*;

#[test]
fn transfer_case() {
    assert_eq!(dose_total(&[0, 127]), Ok(127));
}
