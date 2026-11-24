use task_2339::*;

#[test]
fn public_examples() {
    assert_eq!(batch_variation_longest_run(&[]), 0);
    assert_eq!(batch_variation_longest_run(&[4]), 1);
}
