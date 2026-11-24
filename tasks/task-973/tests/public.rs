use task_973::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_field_pipe(vec!["first\n123", "header\n猫|123"]),
        vec![3usize, 3usize]
    );
    assert_eq!(closing_field_pipe(vec![]), vec![]);
}
