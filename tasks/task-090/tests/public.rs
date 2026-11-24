use task_090::*;

#[test]
fn public_examples() {
    assert_eq!(ticket_text_5(4, 0), "B-5/4");
    assert_eq!(ticket_text_5(13, 1), "B-5/12");
}
