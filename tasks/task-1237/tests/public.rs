use task_1237::*;

#[test]
fn public_examples() {
    assert_eq!(seek_offsets_distinct(&[]), vec![]);
    assert_eq!(seek_offsets_distinct(&[4]), vec![]);
}
