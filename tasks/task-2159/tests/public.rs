use task_2159::*;

#[test]
fn public_examples() {
    assert_eq!(linear_search_probes_longest_run(&[]), 0);
    assert_eq!(linear_search_probes_longest_run(&[4]), 1);
}
