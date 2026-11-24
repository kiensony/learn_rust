use task_625::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_slash(&Record {
            label: String::from("path/1/12"),
            revision: 0
        }),
        1
    );
    assert_eq!(
        shortest_field_with_first_tie_slash(&Record {
            label: String::from("path/123"),
            revision: 1
        }),
        4
    );
}
