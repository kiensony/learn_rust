use task_2352::*;

#[test]
fn public_examples() {
    assert_eq!(single_pass_prefix_maximum_total(&[]), 0);
    assert_eq!(single_pass_prefix_maximum_total(&[4]), 4);
}
