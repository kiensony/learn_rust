use task_835::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_equals("72;12"), Ok(2));
    assert_eq!(leading_marks_removed_equals("猫=12;note"), Ok(6));
}
