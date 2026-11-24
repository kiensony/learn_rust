use task_2976::*;

#[test]
fn public_examples() {
    assert_eq!(queue_depth_descending_steps(&[]), 0);
    assert_eq!(queue_depth_descending_steps(&[4]), 0);
}
