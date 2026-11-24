use task_100::*;

#[test]
fn public_examples() {
    assert_eq!(ticket_text_8(4, 0), "B-8/4");
    assert_eq!(ticket_text_8(13, 1), "B-8/12");
}
