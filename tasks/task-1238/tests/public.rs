use task_1238::*;

#[test]
fn public_examples() {
    assert_eq!(seek_offsets_steps(&[]), vec![]);
    assert_eq!(seek_offsets_steps(&[4]), vec![]);
}
