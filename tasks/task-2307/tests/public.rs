use task_2307::*;

#[test]
fn public_examples() {
    assert_eq!(warmup_discard_distinct(&[]), vec![]);
    assert_eq!(warmup_discard_distinct(&[4]), vec![]);
}
