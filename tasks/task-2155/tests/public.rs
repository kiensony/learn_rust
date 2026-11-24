use task_2155::*;

#[test]
fn public_examples() {
    assert_eq!(linear_search_probes_changes(&[]), vec![]);
    assert_eq!(linear_search_probes_changes(&[4]), vec![1u32]);
}
