use task_209::*;

#[test]
fn public_examples() {
    assert_eq!(status_lamp_10(65535, 4), "low");
    assert_eq!(status_lamp_10(13, 14), "full");
}
