use task_2608::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_checksum_steps(&[]), vec![]);
    assert_eq!(embedded_checksum_steps(&[4]), vec![]);
}
