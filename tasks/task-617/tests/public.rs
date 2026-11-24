use task_617::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_field_slash(&Record {
            label: String::from("path/1/12"),
            revision: 0
        }),
        2
    );
    assert_eq!(
        closing_field_slash(&Record {
            label: String::from("path/123"),
            revision: 1
        }),
        4
    );
}
