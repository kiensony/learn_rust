use task_919::*;

#[test]
fn public_examples() {
    assert_eq!(
        prefix_before_final_mark_slash("path/1/12", |s| s.chars().count() + 1),
        2
    );
    assert_eq!(
        prefix_before_final_mark_slash("path/123", |s| s.len() * 2),
        0
    );
}
