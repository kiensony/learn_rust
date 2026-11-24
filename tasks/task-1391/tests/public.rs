use task_1391::*;

#[test]
fn public_examples() {
    assert_eq!(frame_maxima_trace(&[]), vec![]);
    assert_eq!(frame_maxima_trace(&[0]), vec![0u32]);
}
