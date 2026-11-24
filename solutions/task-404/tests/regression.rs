use task_404::*;

#[test]
fn transfer_case() {
    assert_eq!(has_header(&[255], &[]), true);
}
