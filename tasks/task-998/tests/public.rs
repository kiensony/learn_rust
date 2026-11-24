use task_998::*;

#[test]
fn public_examples() {
    assert_eq!(runs(vec![3, 3, 1, 1, 3]), vec![(3, 2), (1, 2), (3, 1)]);
    assert_eq!(runs(vec![7]), vec![(7, 1)]);
}
