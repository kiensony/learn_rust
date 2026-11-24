use task_126::*;

#[test]
fn public_examples() {
    assert_eq!(saturating_debit_3(4, 0), 1);
    assert_eq!(saturating_debit_3(13, 3), 13);
}
