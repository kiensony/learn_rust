use task_136::*;

#[test]
fn public_examples() {
    assert_eq!(saturating_debit_5(4, 0), 0);
    assert_eq!(saturating_debit_5(13, 1), 7);
}
