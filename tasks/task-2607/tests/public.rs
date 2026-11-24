use task_2607::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_checksum_distinct(&[]), vec![]);
    assert_eq!(embedded_checksum_distinct(&[4]), vec![2u32]);
}
