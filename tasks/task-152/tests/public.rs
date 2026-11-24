use task_152::*;

#[test]
fn public_examples() {
    assert_eq!(price_in_cents_10(65535, 4), 40);
    assert_eq!(price_in_cents_10(13, 14), 130);
}
