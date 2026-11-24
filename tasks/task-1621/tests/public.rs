use task_1621::*;

#[test]
fn public_examples() {
    assert_eq!(fastest(&[]), None);
    assert_eq!(fastest(&[(12, 2), (3, 4)]), Some(4));
}
