use task_2302::*;

#[test]
fn public_examples() {
    assert_eq!(warmup_discard_total(&[]), 0);
    assert_eq!(warmup_discard_total(&[4]), 0);
}
