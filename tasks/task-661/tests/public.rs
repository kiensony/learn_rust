use task_661::*;

#[test]
fn public_examples() {
    assert_eq!(
        prefix_before_final_mark_colon(Event::Data("hi:kit")),
        Some(2)
    );
    assert_eq!(prefix_before_final_mark_colon(Event::Data(":é:a")), Some(3));
}
