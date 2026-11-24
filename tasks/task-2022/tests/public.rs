use task_2022::*;

#[test]
fn public_examples() {
    assert_eq!(polled_groups_total(&[]), 0);
    assert_eq!(polled_groups_total(&[4]), 1);
}
