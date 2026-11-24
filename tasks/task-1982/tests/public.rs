use task_1982::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_record_high_total(&[]), 0);
    assert_eq!(atomic_record_high_total(&[4]), 0);
}
