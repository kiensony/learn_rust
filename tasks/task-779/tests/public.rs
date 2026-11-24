use task_779::*;

#[test]
fn public_examples() {
    assert_eq!(unique(&[&[2, 2], &[3, 2]]), 2);
    assert_eq!(unique(&[]), 0);
}
