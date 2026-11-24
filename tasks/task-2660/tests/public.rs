use task_2660::*;

#[test]
fn public_examples() {
    assert_eq!(lending_windows_reverse(&[]), vec![]);
    assert_eq!(lending_windows_reverse(&[4]), vec![1u32]);
}
