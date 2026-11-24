use task_179::*;

#[test]
fn public_examples() {
    assert_eq!(status_lamp_3(4, 0), "ready");
    assert_eq!(status_lamp_3(13, 3), "full");
}
