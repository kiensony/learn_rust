use task_899::*;

#[test]
fn transfer_case() {
    assert_eq!(wire_value(Mode::Reserved(2)), 2);
}
