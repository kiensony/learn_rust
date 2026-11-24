use task_425::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("path/1/12");
            first_nonempty_field_slash(&mut data);
            data
        },
        "1"
    );
    assert_eq!(
        {
            let mut data = String::from("path/123");
            first_nonempty_field_slash(&mut data);
            data
        },
        "123"
    );
}
