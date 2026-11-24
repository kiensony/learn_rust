use task_253::*;

#[test]
fn public_examples() {
    assert_eq!(running_deposits_10(255, 4), 100);
    assert_eq!(running_deposits_10(13, 14), 910);
}
