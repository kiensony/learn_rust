use task_1003::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_colon(["hi:kit", ":é:a"]), [3, 1]);
    assert_eq!(
        closing_field_colon([String::from(":é:a"), String::from("")]),
        [1, 0]
    );
}
