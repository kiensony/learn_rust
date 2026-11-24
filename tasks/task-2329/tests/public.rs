use task_2329::*;

#[test]
fn public_examples() {
    assert_eq!(batch_medians_longest_run(&[]), 0);
    assert_eq!(batch_medians_longest_run(&[4]), 1);
}
