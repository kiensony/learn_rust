use task_213::*;

#[test]
fn public_examples() {
    assert_eq!(running_deposits_2(4), 20);
    assert_eq!(running_deposits_2(13), 182);
}
