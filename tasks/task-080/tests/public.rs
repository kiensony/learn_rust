use task_080::*;

#[test]
fn public_examples() {
    assert_eq!(ticket_text_3(4, 0), "B-3/4");
    assert_eq!(ticket_text_3(13, 3), "B-3/16");
}
