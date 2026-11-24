use task_422::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("path/1/12");
            marks_removed_at_both_ends_slash(&mut data);
            data
        },
        "1/12"
    );
    assert_eq!(
        {
            let mut data = String::from("path/123");
            marks_removed_at_both_ends_slash(&mut data);
            data
        },
        "123"
    );
}
