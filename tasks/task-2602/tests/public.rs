use task_2602::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_checksum_total(&[]), 0);
    assert_eq!(embedded_checksum_total(&[4]), 2);
}
