use task_2391::*;

#[test]
fn public_examples() {
    assert_eq!(last_change_age_trace(&[]), vec![]);
    assert_eq!(last_change_age_trace(&[4]), vec![1u32]);
}
