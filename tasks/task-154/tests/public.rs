use task_154::*;

#[test]
fn public_examples() {
    assert_eq!(round_up_10(65535, 4), 10);
    assert_eq!(round_up_10(13, 14), 20);
}
