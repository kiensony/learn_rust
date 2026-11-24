use task_2151::*;

#[test]
fn public_examples() {
    assert_eq!(linear_search_probes_trace(&[]), vec![]);
    assert_eq!(linear_search_probes_trace(&[4]), vec![1u32]);
}
