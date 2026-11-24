use task_749::*;

#[test]
fn leading_zeroes_inside_the_selected_field_are_valid_digits() {
    assert_eq!(
        remainder_after_first_mark_hash(Some("prefix[key#0008]suffix")),
        Some(8)
    );
}
