use task_2392::*;

#[test]
fn public_examples() {
    assert_eq!(last_change_age_total(&[]), 0);
    assert_eq!(last_change_age_total(&[4]), 1);
}
