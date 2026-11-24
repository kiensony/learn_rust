use task_1967::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_bit_union_distinct(&[]), vec![]);
    assert_eq!(atomic_bit_union_distinct(&[4]), vec![0u32]);
}
