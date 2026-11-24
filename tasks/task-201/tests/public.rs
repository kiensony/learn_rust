use task_201::*;

#[test]
fn public_examples() {
    assert_eq!(last_ticket_10(65535, 4), true);
    assert_eq!(last_ticket_10(13, 14), false);
}
