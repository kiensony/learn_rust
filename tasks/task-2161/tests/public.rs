use task_2161::*;

#[test]
fn public_examples() {
    assert_eq!(sorted_insertion_moves_trace(&[]), vec![]);
    assert_eq!(sorted_insertion_moves_trace(&[4]), vec![0u32]);
}
