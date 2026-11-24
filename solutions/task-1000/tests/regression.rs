use task_1000::*;

#[test]
fn transfer_case() {
    assert_eq!(
        longest_field_with_last_tie_hash(vec!["", "#", "#"]),
        vec![0usize, 0usize, 0usize]
    );
}
