use task_1147::*;

#[test]
fn public_examples() {
    assert_eq!(apply_all(3, &[&Offset(2), &Clamp(4)]), 4);
    assert_eq!(apply_all(3, &[&Clamp(4), &Offset(2)]), 5);
}
