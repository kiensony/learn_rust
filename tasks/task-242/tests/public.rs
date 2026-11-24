use task_242::*;

#[test]
fn public_examples() {
    assert_eq!(skip_reserved_seats_8(4, 0), 4);
    assert_eq!(skip_reserved_seats_8(13, 1), 11);
}
