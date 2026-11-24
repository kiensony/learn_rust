use task_1954::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_running_total_peaks(&[]), vec![]);
    assert_eq!(atomic_running_total_peaks(&[4]), vec![0u32]);
}
