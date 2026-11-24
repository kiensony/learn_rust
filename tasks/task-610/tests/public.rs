use task_610::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_colon(&Record {
            label: String::from("hi:kit"),
            revision: 0
        }),
        6
    );
    assert_eq!(
        closing_marks_removed_colon(&Record {
            label: String::from(""),
            revision: 2
        }),
        2
    );
}
