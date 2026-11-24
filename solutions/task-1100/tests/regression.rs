use task_1100::*;

#[test]
fn transfer_case() {
    assert_eq!(
        shortest_field_with_first_tie_hash(&String::from("ok#🐈")),
        0
    );
}
