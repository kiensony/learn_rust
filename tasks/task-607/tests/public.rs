use task_607::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_field_colon(&Record {
            label: String::from("hi:kit"),
            revision: 0
        }),
        2
    );
    assert_eq!(
        leading_field_colon(&Record {
            label: String::from(":é:a"),
            revision: 1
        }),
        1
    );
}
