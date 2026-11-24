use task_070::*;

#[test]
fn public_examples() {
    assert_eq!(ticket_text_2(4), "B-2/4");
    assert_eq!(ticket_text_2(13), "B-2/13");
}
