use task_171::*;

#[test]
fn public_examples() {
    assert_eq!(last_ticket_3(4, 0), false);
    assert_eq!(last_ticket_3(0, 0), true);
}
