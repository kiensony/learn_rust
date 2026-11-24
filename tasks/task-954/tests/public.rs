use task_954::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_field_colon(vec!["hi:kit", ":é:a"]),
        vec![3usize, 1usize]
    );
    assert_eq!(closing_field_colon(vec![":é:a", ""]), vec![1usize, 0usize]);
}
