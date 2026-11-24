use task_2514::*;

#[test]
fn public_examples() {
    assert_eq!(abi_flags_peaks(&[]), vec![]);
    assert_eq!(abi_flags_peaks(&[4]), vec![1u32]);
}
