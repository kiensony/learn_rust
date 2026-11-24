use task_671::*;

#[test]
fn public_examples() {
    assert_eq!(
        prefix_before_final_mark_slash(Event::Data("path/1/12")),
        Some(1)
    );
    assert_eq!(
        prefix_before_final_mark_slash(Event::Data("path/123")),
        Some(0)
    );
}
