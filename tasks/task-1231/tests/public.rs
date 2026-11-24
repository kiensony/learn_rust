use task_1231::*;

#[test]
fn public_examples() {
    assert_eq!(seek_offsets_trace(&[]), vec![]);
    assert_eq!(seek_offsets_trace(&[4]), vec![]);
}
