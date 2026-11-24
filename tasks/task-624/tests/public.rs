use task_624::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_slash(&Record {
            label: String::from("path/1/12"),
            revision: 0
        }),
        2
    );
    assert_eq!(
        longest_field_with_last_tie_slash(&Record {
            label: String::from("path/123"),
            revision: 1
        }),
        4
    );
}
