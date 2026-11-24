use task_111::*;

#[test]
fn public_examples() {
    assert_eq!(change(2, 5), 3);
    assert_eq!(change(4, -2), -6);
}
