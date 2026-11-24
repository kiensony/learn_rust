use task_2215::*;

#[test]
fn public_examples() {
    assert_eq!(borrowed_offsets_changes(""), vec![]);
    assert_eq!(borrowed_offsets_changes("a"), vec![]);
}
