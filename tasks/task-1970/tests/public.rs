use task_1970::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_bit_union_reverse(&[]), vec![]);
    assert_eq!(atomic_bit_union_reverse(&[4]), vec![0u32]);
}
