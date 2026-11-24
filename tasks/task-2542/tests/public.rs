use task_2542::*;

#[test]
fn public_examples() {
    assert_eq!(abi_status_mask_total(&[]), 0);
    assert_eq!(abi_status_mask_total(&[4]), 4);
}
