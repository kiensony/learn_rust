use task_958::*;

#[test]
fn public_examples() {
    assert_eq!(
        prefix_before_final_mark_colon(vec!["hi:kit", ":é:a"]),
        vec![2usize, 3usize]
    );
    assert_eq!(
        prefix_before_final_mark_colon(vec![":é:a", ""]),
        vec![3usize, 0usize]
    );
}
