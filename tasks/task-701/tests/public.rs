use task_701::*;

#[test]
fn public_examples() {
    assert_eq!(position(Motion::Home, 9), 0);
    assert_eq!(position(Motion::Shift(-4), 2), -2);
}
