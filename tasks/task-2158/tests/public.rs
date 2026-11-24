use task_2158::*;

#[test]
fn public_examples() {
    assert_eq!(linear_search_probes_steps(&[]), vec![]);
    assert_eq!(linear_search_probes_steps(&[4]), vec![]);
}
