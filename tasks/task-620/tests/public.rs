use task_620::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_slash(&Record {
            label: String::from("path/1/12"),
            revision: 0
        }),
        4
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(&Record {
            label: String::from("hi/kit"),
            revision: 2
        }),
        8
    );
}
