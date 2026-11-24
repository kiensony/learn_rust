use task_1962::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_bit_union_total(&[]), 0);
    assert_eq!(atomic_bit_union_total(&[4]), 0);
}
