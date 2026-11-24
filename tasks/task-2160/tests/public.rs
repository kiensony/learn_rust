use task_2160::*;

#[test]
fn public_examples() {
    assert_eq!(linear_search_probes_reverse(&[]), vec![]);
    assert_eq!(linear_search_probes_reverse(&[4]), vec![1u32]);
}
