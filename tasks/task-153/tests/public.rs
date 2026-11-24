use task_153::*;

#[test]
fn public_examples() {
    assert_eq!(round_down_10(65535, 4), 0);
    assert_eq!(round_down_10(13, 14), 10);
}
