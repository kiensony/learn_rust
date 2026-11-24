use task_175::*;

#[test]
fn public_examples() {
    assert_eq!(service_interval_3(4, 0), false);
    assert_eq!(service_interval_3(0, 6), true);
}
