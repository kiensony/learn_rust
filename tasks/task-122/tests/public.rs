use task_122::*;

#[test]
fn public_examples() {
    assert_eq!(price_in_cents_3(4, 0), 12);
    assert_eq!(price_in_cents_3(13, 3), 48);
}
