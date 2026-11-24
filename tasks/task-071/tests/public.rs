use task_071::*;

#[test]
fn public_examples() {
    assert_eq!(ticket_mark('A'), ('A', false));
    assert_eq!(ticket_mark('*'), ('*', true));
}
