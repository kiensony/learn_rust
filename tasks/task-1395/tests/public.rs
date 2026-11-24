use task_1395::*;

#[test]
fn public_examples() {
    assert_eq!(frame_maxima_changes(&[]), vec![]);
    assert_eq!(frame_maxima_changes(&[0]), vec![0u32]);
}
