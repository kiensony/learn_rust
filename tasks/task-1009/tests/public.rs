use task_1009::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_colon(["hi:kit", ":é:a"]), [2, 2]);
    assert_eq!(
        first_nonempty_field_colon([String::from(":é:a"), String::from("")]),
        [2, 0]
    );
}
