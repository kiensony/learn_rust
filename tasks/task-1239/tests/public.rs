use task_1239::*;

#[test]
fn public_examples() {
    assert_eq!(seek_offsets_longest_run(&[]), 0);
    assert_eq!(seek_offsets_longest_run(&[4]), 0);
}
