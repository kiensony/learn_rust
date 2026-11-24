use task_2639::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_saturation_longest_run(&[]), 0);
    assert_eq!(embedded_saturation_longest_run(&[4]), 1);
}
