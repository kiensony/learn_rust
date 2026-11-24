use task_336::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_pipe(String::from("first\n123")),
        "123"
    );
    assert_eq!(
        longest_field_with_last_tie_pipe(String::from("hi|kit")),
        "kit"
    );
}
