use task_2655::*;

#[test]
fn public_examples() {
    assert_eq!(lending_windows_changes(&[]), vec![]);
    assert_eq!(lending_windows_changes(&[4]), vec![1u32]);
}
