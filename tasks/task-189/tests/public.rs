use task_189::*;

#[test]
fn public_examples() {
    assert_eq!(status_lamp_5(4, 0), "low");
    assert_eq!(status_lamp_5(13, 1), "full");
}
