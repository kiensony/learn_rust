use task_2803::*;

#[test]
fn public_examples() {
    assert_eq!(service_acknowledgments_bounds(&[]), None);
    assert_eq!(service_acknowledgments_bounds(&[4]), Some((4u32, 4u32)));
}
