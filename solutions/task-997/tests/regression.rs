use task_997::*;

#[test]
fn transfer_case() {
    assert_eq!(
        remainder_after_first_mark_hash(vec!["", "#", "#"]),
        vec![0usize, 0usize, 0usize]
    );
}
