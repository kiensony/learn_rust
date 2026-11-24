use task_2245::*;

#[test]
fn public_examples() {
    assert_eq!(borrowed_windows_changes(""), vec![]);
    assert_eq!(borrowed_windows_changes("a"), vec![]);
}
