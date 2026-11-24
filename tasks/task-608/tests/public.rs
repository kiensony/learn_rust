use task_608::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_field_colon(&Record {
            label: String::from("hi:kit"),
            revision: 0
        }),
        3
    );
    assert_eq!(
        closing_field_colon(&Record {
            label: String::from(":é:a"),
            revision: 1
        }),
        2
    );
}
