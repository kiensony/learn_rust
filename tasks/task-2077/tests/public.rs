use task_2077::*;

#[test]
fn public_examples() {
    assert_eq!(cancel_reservation(4, 9), vec![4, 9]);
    assert_eq!(cancel_reservation(1, 1), vec![1, 1]);
}
