use task_232::*;

#[test]
fn public_examples() {
    assert_eq!(skip_reserved_seats_5(4, 0), 4);
    assert_eq!(skip_reserved_seats_5(13, 1), 10);
}
