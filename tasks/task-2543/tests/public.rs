use task_2543::*;

#[test]
fn public_examples() {
    assert_eq!(abi_status_mask_bounds(&[]), None);
    assert_eq!(abi_status_mask_bounds(&[4]), Some((4u32, 4u32)));
}
