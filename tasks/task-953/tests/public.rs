use task_953::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_field_colon(vec!["hi:kit", ":é:a"]),
        vec![2usize, 0usize]
    );
    assert_eq!(leading_field_colon(vec![":é:a", ""]), vec![0usize, 0usize]);
}
