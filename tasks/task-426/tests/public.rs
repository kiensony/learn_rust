use task_426::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("path/1/12");
            longest_field_with_last_tie_slash(&mut data);
            data
        },
        "12"
    );
    assert_eq!(
        {
            let mut data = String::from("path/123");
            longest_field_with_last_tie_slash(&mut data);
            data
        },
        "123"
    );
}
