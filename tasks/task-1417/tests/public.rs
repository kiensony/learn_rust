use task_1417::*;

#[test]
fn public_examples() {
    assert_eq!(elapsed_seconds_distinct(&[]), vec![]);
    assert_eq!(elapsed_seconds_distinct(&[4]), vec![0u32]);
}
