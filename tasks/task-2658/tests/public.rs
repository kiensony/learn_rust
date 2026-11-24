use task_2658::*;

#[test]
fn public_examples() {
    assert_eq!(lending_windows_steps(&[]), vec![]);
    assert_eq!(lending_windows_steps(&[4]), vec![]);
}
