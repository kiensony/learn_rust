use task_419::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("path/1/12");
            closing_field_slash(&mut data);
            data
        },
        "12"
    );
    assert_eq!(
        {
            let mut data = String::from("path/123");
            closing_field_slash(&mut data);
            data
        },
        "123"
    );
}
