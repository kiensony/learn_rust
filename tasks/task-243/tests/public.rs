use task_243::*;

#[test]
fn public_examples() {
    assert_eq!(running_deposits_8(4, 0), 80);
    assert_eq!(running_deposits_8(13, 1), 624);
}
