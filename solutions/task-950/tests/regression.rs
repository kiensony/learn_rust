use task_950::*;

#[test]
fn transfer_case() {
    assert_eq!(first_nonempty_field_hash("ok#🐈", |s| s.len() * 2), 0);
}
