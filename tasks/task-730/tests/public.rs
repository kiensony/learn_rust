use task_730::*;

#[test]
fn public_examples() {
    assert_eq!(port("80"), Some(80));
    assert_eq!(port("0"), Some(0));
}
