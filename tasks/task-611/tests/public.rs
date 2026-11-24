use task_611::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_colon(&Record {
            label: String::from("hi:kit"),
            revision: 0
        }),
        6
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(&Record {
            label: String::from(":é:a"),
            revision: 1
        }),
        5
    );
}
