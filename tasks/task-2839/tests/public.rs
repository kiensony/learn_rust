use task_2839::*;

#[test]
fn public_examples() {
    assert_eq!(service_expansion_longest_run(&[]), 0);
    assert_eq!(service_expansion_longest_run(&[4]), 1);
}
