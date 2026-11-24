use task_3000::*;

#[test]
fn public_examples() {
    assert_eq!(affected_ticks(&[]), 0);
    assert_eq!(affected_ticks(&[(1, 4), (3, 6)]), 5);
}
