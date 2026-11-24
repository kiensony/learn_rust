use task_2308::*;

#[test]
fn public_examples() {
    assert_eq!(warmup_discard_steps(&[]), vec![]);
    assert_eq!(warmup_discard_steps(&[4]), vec![]);
}
