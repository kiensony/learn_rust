use task_406::*;

#[test]
fn transfer_case() {
    assert_eq!(longest_field_with_last_tie_hash(&String::from("ok#🐈")), 0);
}
