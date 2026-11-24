use task_376::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_slash(&String::from("path/1/12")),
        2
    );
    assert_eq!(
        longest_field_with_last_tie_slash(&String::from("path/123")),
        3
    );
}
