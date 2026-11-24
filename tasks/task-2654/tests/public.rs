use task_2654::*;

#[test]
fn public_examples() {
    assert_eq!(lending_windows_peaks(&[]), vec![]);
    assert_eq!(lending_windows_peaks(&[4]), vec![1u32]);
}
