use task_1764::*;

#[test]
fn public_examples() {
    assert_eq!(generated_offset_peaks(&[]), vec![]);
    assert_eq!(generated_offset_peaks(&[4]), vec![11u32]);
}
