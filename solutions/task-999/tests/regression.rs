use task_999::*;

#[test]
fn transfer_case() {
    assert_eq!(
        first_nonempty_field_hash(vec!["", "#", "#"]),
        vec![0usize, 0usize, 0usize]
    );
}
