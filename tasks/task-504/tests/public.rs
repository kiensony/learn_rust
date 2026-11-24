use task_504::*;

#[test]
fn public_examples() {
    assert_eq!(columns(&[(2, 3), (4, 5)]), (vec![2, 4], vec![3, 5]));
    assert_eq!(columns(&[(1, -1)]), (vec![1], vec![-1]));
}
