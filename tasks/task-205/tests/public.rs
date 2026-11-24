use task_205::*;

#[test]
fn public_examples() {
    assert_eq!(service_interval_10(65535, 4), false);
    assert_eq!(service_interval_10(10, 11), true);
}
