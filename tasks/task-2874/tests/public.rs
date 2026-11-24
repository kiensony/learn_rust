use task_2874::*;

#[test]
fn public_examples() {
    assert_eq!(flag_register_peaks(&[]), vec![]);
    assert_eq!(flag_register_peaks(&[4]), vec![]);
}
