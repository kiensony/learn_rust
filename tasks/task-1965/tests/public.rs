use task_1965::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_bit_union_changes(&[]), vec![]);
    assert_eq!(atomic_bit_union_changes(&[4]), vec![0u32]);
}
