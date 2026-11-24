use task_1873::*;

#[test]
fn public_examples() {
    assert_eq!(shared_payload_bounds(&[]), None);
    assert_eq!(shared_payload_bounds(&[4]), Some((4u32, 4u32)));
}
