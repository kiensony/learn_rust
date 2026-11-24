use task_2544::*;

#[test]
fn public_examples() {
    assert_eq!(abi_status_mask_peaks(&[]), vec![]);
    assert_eq!(abi_status_mask_peaks(&[4]), vec![4u32]);
}
