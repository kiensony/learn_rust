use task_1235::*;

#[test]
fn public_examples() {
    assert_eq!(seek_offsets_changes(&[]), vec![]);
    assert_eq!(seek_offsets_changes(&[4]), vec![]);
}
