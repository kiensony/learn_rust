use task_2614::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_flags_peaks(&[]), vec![]);
    assert_eq!(embedded_flags_peaks(&[4]), vec![4u32]);
}
