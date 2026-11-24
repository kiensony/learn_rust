use task_2904::*;

#[test]
fn public_examples() {
    assert_eq!(latency_increases_peaks(&[]), vec![]);
    assert_eq!(latency_increases_peaks(&[4]), vec![]);
}
