use task_998::*;

#[test]
fn transfer_case() {
    assert_eq!(runs(vec![1, 2, 1, 2]), vec![(1, 1), (2, 1), (1, 1), (2, 1)]);
}
