use task_2676::*;

#[test]
fn public_examples() {
    assert_eq!(lending_checksum_descending_steps(&[]), 0);
    assert_eq!(lending_checksum_descending_steps(&[4]), 0);
}
