use task_619::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_slash(&Record {
            label: String::from("path/1/12"),
            revision: 0
        }),
        4
    );
    assert_eq!(
        closing_marks_removed_slash(&Record {
            label: String::from("hi/kit"),
            revision: 2
        }),
        8
    );
}
