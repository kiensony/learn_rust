use task_2843::*;

#[test]
fn public_examples() {
    assert_eq!(service_status_bounds(&[]), None);
    assert_eq!(service_status_bounds(&[4]), Some((0u32, 0u32)));
}
