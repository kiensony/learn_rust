use task_233::*;

#[test]
fn public_examples() {
    assert_eq!(running_deposits_5(4, 0), 50);
    assert_eq!(running_deposits_5(13, 1), 390);
}
