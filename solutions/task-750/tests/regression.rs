use task_750::*;

#[test]
fn transfer_case() {
    assert_eq!(first_ok(&[Ok(0), Err(())]), Some(0));
}
