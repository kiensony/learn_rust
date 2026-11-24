use task_1200::*;

#[test]
fn transfer_case() {
    assert_eq!(
        shortest_field_with_first_tie_hash("ok#🐈", "backup"),
        "backup"
    );
}
