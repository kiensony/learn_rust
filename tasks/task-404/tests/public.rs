use task_404::*;

#[test]
fn public_examples() {
    assert_eq!(has_header(&[2, 3, 4], &[2, 3]), true);
    assert_eq!(has_header(&[2, 4], &[4]), false);
}
