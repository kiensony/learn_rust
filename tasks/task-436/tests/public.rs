use task_436::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("first\n123");
            longest_field_with_last_tie_pipe(&mut data);
            data
        },
        "123"
    );
    assert_eq!(
        {
            let mut data = String::from("hi|kit");
            longest_field_with_last_tie_pipe(&mut data);
            data
        },
        "kit"
    );
}
