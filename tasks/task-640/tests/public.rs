use task_640::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_equals(&Record {
            label: String::from("72;12"),
            revision: 0
        }),
        2
    );
    assert_eq!(
        marks_removed_at_both_ends_equals(&Record {
            label: String::from("猫=12;note"),
            revision: 1
        }),
        7
    );
}
