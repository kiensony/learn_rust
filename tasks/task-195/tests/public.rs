use task_195::*;

#[test]
fn public_examples() {
    assert_eq!(service_interval_8(4, 0), false);
    assert_eq!(service_interval_8(8, 0), true);
}
