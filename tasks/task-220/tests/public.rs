use task_220::*;

#[test]
fn public_examples() {
    assert_eq!(before_stop(&[1, 2, 255, 4]), 3);
    assert_eq!(before_stop(&[255, 3]), 0);
}
