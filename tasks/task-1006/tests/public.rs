use task_1006::*;

#[test]
fn public_examples() {
    assert_eq!(marks_removed_at_both_ends_colon(["hi:kit", ":é:a"]), [6, 4]);
    assert_eq!(
        marks_removed_at_both_ends_colon([String::from(":é:a"), String::from("")]),
        [4, 0]
    );
}
