use task_2541::*;

#[test]
fn public_examples() {
    assert_eq!(abi_status_mask_trace(&[]), vec![]);
    assert_eq!(abi_status_mask_trace(&[4]), vec![4u32]);
}
