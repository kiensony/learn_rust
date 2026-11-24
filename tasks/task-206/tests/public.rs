use task_206::*;

#[test]
fn public_examples() {
    assert_eq!(admitted(true, false, false), true);
    assert_eq!(admitted(true, true, true), false);
}
