use task_334::*;

#[test]
fn public_examples() {
    assert_eq!(both_points((1, 3)), [(1, 3), (1, 3)]);
    assert_eq!(both_points((-4, 2)), [(-4, 2), (-4, 2)]);
}
