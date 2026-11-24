use task_2157::*;

#[test]
fn public_examples() {
    assert_eq!(linear_search_probes_distinct(&[]), vec![]);
    assert_eq!(linear_search_probes_distinct(&[4]), vec![1u32]);
}
