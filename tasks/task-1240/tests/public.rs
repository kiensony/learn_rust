use task_1240::*;

#[test]
fn public_examples() {
    assert_eq!(seek_offsets_reverse(&[]), vec![]);
    assert_eq!(seek_offsets_reverse(&[4]), vec![]);
}
