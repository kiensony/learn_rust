use task_2604::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_checksum_peaks(&[]), vec![]);
    assert_eq!(embedded_checksum_peaks(&[4]), vec![2u32]);
}
