use task_1933::*;

#[test]
fn public_examples() {
    assert_eq!(finish_queue(&[]), Ok(vec![]));
    assert_eq!(finish_queue(&[1, 2]), Ok(vec![1, 2]));
}
