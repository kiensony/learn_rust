use task_355::*;

#[test]
fn transfer_case() {
    assert_eq!(first_nonempty_field_hash(String::from("ok#🐈")), "");
}
