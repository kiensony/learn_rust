use task_132::*;

#[test]
fn public_examples() {
    assert_eq!(price_in_cents_5(4, 0), 20);
    assert_eq!(price_in_cents_5(13, 1), 60);
}
