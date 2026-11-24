use task_2610::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_checksum_reverse(&[]), vec![]);
    assert_eq!(embedded_checksum_reverse(&[4]), vec![2u32]);
}
