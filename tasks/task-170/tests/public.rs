use task_170::*;

#[test]
fn public_examples() {
    assert_eq!(instruction('R'), "stop");
    assert_eq!(instruction('G'), "go");
}
