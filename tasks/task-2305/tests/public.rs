use task_2305::*;

#[test]
fn public_examples() {
    assert_eq!(warmup_discard_changes(&[]), vec![]);
    assert_eq!(warmup_discard_changes(&[4]), vec![]);
}
