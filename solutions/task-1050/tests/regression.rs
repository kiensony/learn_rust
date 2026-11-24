use task_1050::*;

#[test]
fn transfer_case() {
    assert_eq!(
        longest_field_with_last_tie_hash([String::from(""), String::from("#"), String::from("#")]),
        [0, 0, 0]
    );
}
