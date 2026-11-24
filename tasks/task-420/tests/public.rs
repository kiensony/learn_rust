use task_420::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("path/1/12");
            leading_marks_removed_slash(&mut data);
            data
        },
        "1/12"
    );
    assert_eq!(
        {
            let mut data = String::from("path/123");
            leading_marks_removed_slash(&mut data);
            data
        },
        "123"
    );
}
