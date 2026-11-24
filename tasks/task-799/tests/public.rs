use task_799::*;

#[test]
fn public_examples() {
    assert_eq!(leader(&[4, 2, 4]), Some(4));
    assert_eq!(leader(&[3, 2, 3, 2]), Some(2));
}
