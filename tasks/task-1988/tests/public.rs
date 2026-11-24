use task_1988::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_record_high_steps(&[]), vec![]);
    assert_eq!(atomic_record_high_steps(&[4]), vec![]);
}
