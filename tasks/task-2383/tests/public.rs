use task_2383::*;

#[test]
fn public_examples() {
    assert_eq!(rolling_parity_bounds(&[]), None);
    assert_eq!(rolling_parity_bounds(&[4]), Some((4u32, 4u32)));
}
