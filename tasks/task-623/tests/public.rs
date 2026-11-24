use task_623::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_nonempty_field_slash(&Record {
            label: String::from("path/1/12"),
            revision: 0
        }),
        1
    );
    assert_eq!(
        first_nonempty_field_slash(&Record {
            label: String::from("path/123"),
            revision: 1
        }),
        4
    );
}
