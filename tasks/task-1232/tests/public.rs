use task_1232::*;

#[test]
fn public_examples() {
    assert_eq!(seek_offsets_total(&[]), 0);
    assert_eq!(seek_offsets_total(&[4]), 0);
}
