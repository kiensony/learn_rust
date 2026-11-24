use task_1420::*;

#[test]
fn public_examples() {
    assert_eq!(elapsed_seconds_reverse(&[]), vec![]);
    assert_eq!(elapsed_seconds_reverse(&[4]), vec![0u32]);
}
