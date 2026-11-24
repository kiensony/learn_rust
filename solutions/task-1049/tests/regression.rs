use task_1049::*;

#[test]
fn transfer_case() {
    assert_eq!(
        first_nonempty_field_hash([String::from(""), String::from("#"), String::from("#")]),
        [0, 0, 0]
    );
}
