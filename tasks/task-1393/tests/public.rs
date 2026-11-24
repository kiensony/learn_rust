use task_1393::*;

#[test]
fn public_examples() {
    assert_eq!(frame_maxima_bounds(&[]), None);
    assert_eq!(frame_maxima_bounds(&[0]), Some((0u32, 0u32)));
}
