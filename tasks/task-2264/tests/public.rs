use task_2264::*;

#[test]
fn public_examples() {
    assert_eq!(hot_code_addresses_peaks(&[]), vec![]);
    assert_eq!(hot_code_addresses_peaks(&[4]), vec![]);
}
