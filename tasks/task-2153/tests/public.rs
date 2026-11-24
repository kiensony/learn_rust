use task_2153::*;

#[test]
fn public_examples() {
    assert_eq!(linear_search_probes_bounds(&[]), None);
    assert_eq!(linear_search_probes_bounds(&[4]), Some((1u32, 1u32)));
}
