use task_1969::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_bit_union_longest_run(&[]), 0);
    assert_eq!(atomic_bit_union_longest_run(&[4]), 1);
}
