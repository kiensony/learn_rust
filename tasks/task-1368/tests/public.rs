use task_1368::*;

#[test]
fn public_examples() {
    assert_eq!(frame_offsets_steps(&[]), vec![]);
    assert_eq!(frame_offsets_steps(&[0]), vec![]);
}
