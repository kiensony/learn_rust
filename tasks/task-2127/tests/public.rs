use task_2127::*;

#[test]
fn public_examples() {
    assert_eq!(queue_rotation_distinct(&[]), vec![]);
    assert_eq!(queue_rotation_distinct(&[4]), vec![4u32]);
}
