use task_1964::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_bit_union_peaks(&[]), vec![]);
    assert_eq!(atomic_bit_union_peaks(&[4]), vec![0u32]);
}
