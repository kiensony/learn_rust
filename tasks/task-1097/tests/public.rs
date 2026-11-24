use task_1097::*;

#[test]
fn public_examples() {
    assert_eq!(total_span(&[Length(2), Length(5)]), 7);
    assert_eq!(total_span(&[Interval(4, 9), Interval(9, 4)]), 10);
}
