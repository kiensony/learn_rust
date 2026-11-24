use task_403::*;

#[test]
fn transfer_case() {
    assert_eq!(remainder_after_first_mark_hash(&String::from("ok#🐈")), 0);
}
