use task_2609::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_checksum_longest_run(&[]), 0);
    assert_eq!(embedded_checksum_longest_run(&[4]), 1);
}
