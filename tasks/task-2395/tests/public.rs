use task_2395::*;

#[test]
fn public_examples() {
    assert_eq!(last_change_age_changes(&[]), vec![]);
    assert_eq!(last_change_age_changes(&[4]), vec![1u32]);
}
