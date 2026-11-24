use task_429::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("first\n123");
            closing_field_pipe(&mut data);
            data
        },
        "123"
    );
    assert_eq!(
        {
            let mut data = String::from("hi|kit");
            closing_field_pipe(&mut data);
            data
        },
        "kit"
    );
}
