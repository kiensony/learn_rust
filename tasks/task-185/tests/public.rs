use task_185::*;

#[test]
fn public_examples() {
    assert_eq!(service_interval_5(4, 0), false);
    assert_eq!(service_interval_5(7, 2), true);
}
