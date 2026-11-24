use task_2606::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_checksum_descending_steps(&[]), 0);
    assert_eq!(embedded_checksum_descending_steps(&[4]), 0);
}
