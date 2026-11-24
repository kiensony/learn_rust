use task_2659::*;

#[test]
fn public_examples() {
    assert_eq!(lending_windows_longest_run(&[]), 0);
    assert_eq!(lending_windows_longest_run(&[4]), 1);
}
