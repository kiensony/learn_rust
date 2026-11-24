use task_615::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_colon(&Record {
            label: String::from("hi:kit"),
            revision: 0
        }),
        2
    );
    assert_eq!(
        shortest_field_with_first_tie_colon(&Record {
            label: String::from(":é:a"),
            revision: 1
        }),
        1
    );
}
