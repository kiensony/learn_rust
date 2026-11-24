use task_952::*;

#[test]
fn transfer_case() {
    assert_eq!(
        shortest_field_with_first_tie_hash("ok#🐈", |s| s.len() * 2),
        0
    );
}
