use task_441::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("72;12");
            closing_marks_removed_equals(&mut data);
            data
        },
        "72"
    );
    assert_eq!(
        {
            let mut data = String::from("猫=12;note");
            closing_marks_removed_equals(&mut data);
            data
        },
        "猫=12"
    );
}
