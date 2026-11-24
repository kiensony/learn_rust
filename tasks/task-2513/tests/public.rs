use task_2513::*;

#[test]
fn public_examples() {
    assert_eq!(abi_flags_bounds(&[]), None);
    assert_eq!(abi_flags_bounds(&[4]), Some((1u32, 1u32)));
}
