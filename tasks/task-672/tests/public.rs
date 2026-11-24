use task_672::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_nonempty_field_slash(Event::Data("path/1/12")),
        Some(1)
    );
    assert_eq!(first_nonempty_field_slash(Event::Data("path/123")), Some(3));
}
