use task_222::*;

#[test]
fn public_examples() {
    assert_eq!(skip_reserved_seats_3(4, 0), 3);
    assert_eq!(skip_reserved_seats_3(13, 3), 11);
}
