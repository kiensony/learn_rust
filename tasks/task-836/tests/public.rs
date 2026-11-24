use task_836::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_equals("72;12"), Ok(2));
    assert_eq!(closing_marks_removed_equals("猫=12;note"), Ok(6));
}
