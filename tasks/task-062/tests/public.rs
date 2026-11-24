use task_062::*;

#[test]
fn public_examples() {
    assert_eq!(open_sign(true), true);
    assert_eq!(open_sign(false), false);
}
