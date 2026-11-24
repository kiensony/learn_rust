use task_1098::*;

#[test]
fn transfer_case() {
    assert_eq!(first_nonempty_field_hash(&String::from("ok#🐈")), 0);
}
