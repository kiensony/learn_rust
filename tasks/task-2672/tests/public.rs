use task_2672::*;

#[test]
fn public_examples() {
    assert_eq!(lending_checksum_total(&[]), 0);
    assert_eq!(lending_checksum_total(&[4]), 4);
}
