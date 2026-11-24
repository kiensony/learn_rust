use task_2834::*;

#[test]
fn public_examples() {
    assert_eq!(service_expansion_peaks(&[]), vec![]);
    assert_eq!(service_expansion_peaks(&[4]), vec![8u32]);
}
