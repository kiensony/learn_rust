use task_2679::*;

#[test]
fn public_examples() {
    assert_eq!(lending_checksum_longest_run(&[]), 0);
    assert_eq!(lending_checksum_longest_run(&[4]), 1);
}
