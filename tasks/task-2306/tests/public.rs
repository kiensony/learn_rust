use task_2306::*;

#[test]
fn public_examples() {
    assert_eq!(warmup_discard_descending_steps(&[]), 0);
    assert_eq!(warmup_discard_descending_steps(&[4]), 0);
}
