use task_2601::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_checksum_trace(&[]), vec![]);
    assert_eq!(embedded_checksum_trace(&[4]), vec![2u32]);
}
