use task_434::*;

#[test]
fn public_examples() {
    assert_eq!(append_reading(vec![3, 6], 2), (vec![3, 6, 2], Some(3)));
    assert_eq!(append_reading(vec![1], 4), (vec![1, 4], Some(1)));
}
