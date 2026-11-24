use task_1734::*;

#[test]
fn public_examples() {
    assert_eq!(population_macro_peaks(&[]), vec![]);
    assert_eq!(population_macro_peaks(&[4]), vec![1u32]);
}
