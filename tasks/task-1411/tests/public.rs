use task_1411::*;

#[test]
fn public_examples() {
    assert_eq!(elapsed_seconds_trace(&[]), vec![]);
    assert_eq!(elapsed_seconds_trace(&[4]), vec![0u32]);
}
