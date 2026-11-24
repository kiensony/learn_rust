use task_639::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_equals(&Record {
            label: String::from("72;12"),
            revision: 0
        }),
        2
    );
    assert_eq!(
        closing_marks_removed_equals(&Record {
            label: String::from("猫=12;note"),
            revision: 1
        }),
        7
    );
}
