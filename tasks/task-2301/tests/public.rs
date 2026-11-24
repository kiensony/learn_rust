use task_2301::*;

#[test]
fn public_examples() {
    assert_eq!(warmup_discard_trace(&[]), vec![]);
    assert_eq!(warmup_discard_trace(&[4]), vec![]);
}
