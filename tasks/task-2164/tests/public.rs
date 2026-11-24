use task_2164::*;

#[test]
fn public_examples() {
    assert_eq!(sorted_insertion_moves_peaks(&[]), vec![]);
    assert_eq!(sorted_insertion_moves_peaks(&[4]), vec![0u32]);
}
