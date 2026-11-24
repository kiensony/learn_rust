use task_1986::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_record_high_descending_steps(&[]), 0);
    assert_eq!(atomic_record_high_descending_steps(&[4]), 0);
}
