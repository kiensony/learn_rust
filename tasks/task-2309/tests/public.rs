use task_2309::*;

#[test]
fn public_examples() {
    assert_eq!(warmup_discard_longest_run(&[]), 0);
    assert_eq!(warmup_discard_longest_run(&[4]), 0);
}
