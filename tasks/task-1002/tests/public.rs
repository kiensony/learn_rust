use task_1002::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_colon(["hi:kit", ":é:a"]), [2, 0]);
    assert_eq!(
        leading_field_colon([String::from(":é:a"), String::from("")]),
        [0, 0]
    );
}
