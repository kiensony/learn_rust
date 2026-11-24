use task_1779::*;

#[test]
fn public_examples() {
    assert_eq!(build_mask_longest_run(&[]), 0);
    assert_eq!(build_mask_longest_run(&[4]), 1);
}
