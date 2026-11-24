use task_2152::*;

#[test]
fn public_examples() {
    assert_eq!(linear_search_probes_total(&[]), 0);
    assert_eq!(linear_search_probes_total(&[4]), 1);
}
