use task_978::*;

#[test]
fn public_examples() {
    assert_eq!(negative_at(&[1, -2, -5]), Some(1));
    assert_eq!(negative_at(&[0, 4]), None);
}
