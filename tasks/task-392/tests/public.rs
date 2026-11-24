use task_392::*;

#[test]
fn public_examples() {
    assert_eq!(marks_removed_at_both_ends_equals(&String::from("72;12")), 2);
    assert_eq!(
        marks_removed_at_both_ends_equals(&String::from("猫=12;note")),
        6
    );
}
