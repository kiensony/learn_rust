use task_2154::*;

#[test]
fn public_examples() {
    assert_eq!(linear_search_probes_peaks(&[]), vec![]);
    assert_eq!(linear_search_probes_peaks(&[4]), vec![1u32]);
}
