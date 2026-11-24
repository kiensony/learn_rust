use task_202::*;

#[test]
fn public_examples() {
    assert_eq!(comfort_zone_10(65535, 4), false);
    assert_eq!(comfort_zone_10(10, 11), true);
}
