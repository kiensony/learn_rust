use task_2914::*;

#[test]
fn public_examples() {
    assert_eq!(latency_improvement_peaks(&[]), vec![]);
    assert_eq!(latency_improvement_peaks(&[4]), vec![]);
}
