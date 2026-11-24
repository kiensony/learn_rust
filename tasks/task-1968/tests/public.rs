use task_1968::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_bit_union_steps(&[]), vec![]);
    assert_eq!(atomic_bit_union_steps(&[4]), vec![]);
}
