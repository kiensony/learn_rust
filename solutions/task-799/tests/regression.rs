use task_799::*;

#[test]
fn transfer_case() {
    assert_eq!(leader(&[1, 2, 2, 1, 3]), Some(1));
}
