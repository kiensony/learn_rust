use task_252::*;

#[test]
fn public_examples() {
    assert_eq!(skip_reserved_seats_10(255, 4), 4);
    assert_eq!(skip_reserved_seats_10(13, 14), 12);
}
