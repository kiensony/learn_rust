use task_1883::*;

#[test]
fn public_examples() {
    assert_eq!(unique_owner_state_bounds(&[]), None);
    assert_eq!(unique_owner_state_bounds(&[4]), Some((1u32, 1u32)));
}
