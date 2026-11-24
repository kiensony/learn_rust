use task_1007::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_colon(["hi:kit", ":é:a"]), [3, 4]);
    assert_eq!(
        remainder_after_first_mark_colon([String::from(":é:a"), String::from("")]),
        [4, 0]
    );
}
