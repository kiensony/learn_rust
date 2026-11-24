use task_199::*;

#[test]
fn public_examples() {
    assert_eq!(status_lamp_8(4, 0), "low");
    assert_eq!(status_lamp_8(13, 1), "full");
}
