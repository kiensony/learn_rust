use task_1963::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_bit_union_bounds(&[]), None);
    assert_eq!(atomic_bit_union_bounds(&[4]), Some((0u32, 0u32)));
}
