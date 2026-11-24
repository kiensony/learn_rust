use task_2999::*;

#[test]
fn public_examples() {
    assert_eq!(handoff(vec![4, 5, 6], 2), (vec![4, 5], vec![6]));
    assert_eq!(handoff(vec![9, 9], 0), (vec![], vec![9, 9]));
}
