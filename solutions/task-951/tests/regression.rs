use task_951::*;

#[test]
fn transfer_case() {
    assert_eq!(
        longest_field_with_last_tie_hash("ok#🐈", |s| s.len() * 2),
        0
    );
}
