use task_899::*;

#[test]
fn public_examples() {
    assert_eq!(wire_value(Mode::Listen), 1);
    assert_eq!(wire_value(Mode::Talk), 2);
}
