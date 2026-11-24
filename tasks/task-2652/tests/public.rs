use task_2652::*;

#[test]
fn public_examples() {
    assert_eq!(lending_windows_total(&[]), 0);
    assert_eq!(lending_windows_total(&[4]), 1);
}
