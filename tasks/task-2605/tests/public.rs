use task_2605::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_checksum_changes(&[]), vec![]);
    assert_eq!(embedded_checksum_changes(&[4]), vec![2u32]);
}
