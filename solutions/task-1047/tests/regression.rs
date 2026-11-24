use task_1047::*;

#[test]
fn transfer_case() {
    assert_eq!(
        remainder_after_first_mark_hash([String::from(""), String::from("#"), String::from("#")]),
        [0, 0, 0]
    );
}
