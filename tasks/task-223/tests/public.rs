use task_223::*;

#[test]
fn public_examples() {
    assert_eq!(running_deposits_3(4, 0), 30);
    assert_eq!(running_deposits_3(13, 3), 408);
}
