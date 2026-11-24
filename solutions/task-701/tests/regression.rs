use task_701::*;

#[test]
fn transfer_case() {
    assert_eq!(position(Motion::Shift(-1), -32768), -32768);
}
