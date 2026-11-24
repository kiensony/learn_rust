use task_642::*;

#[test]
fn public_examples() {
    assert_eq!(
        prefix_before_final_mark_equals(&Record {
            label: String::from("72;12"),
            revision: 0
        }),
        0
    );
    assert_eq!(
        prefix_before_final_mark_equals(&Record {
            label: String::from("猫=12;note"),
            revision: 1
        }),
        4
    );
}
