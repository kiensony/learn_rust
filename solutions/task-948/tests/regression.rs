use task_948::*;

#[test]
fn transfer_case() {
    assert_eq!(remainder_after_first_mark_hash("ok#🐈", |s| s.len() * 2), 0);
}
