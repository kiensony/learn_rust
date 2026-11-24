use task_169::*;

#[test]
fn public_examples() {
    assert_eq!(status_lamp_2(4), "ready");
    assert_eq!(status_lamp_2(13), "full");
}
