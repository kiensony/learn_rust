use task_2651::*;

#[test]
fn public_examples() {
    assert_eq!(lending_windows_trace(&[]), vec![]);
    assert_eq!(lending_windows_trace(&[4]), vec![1u32]);
}
