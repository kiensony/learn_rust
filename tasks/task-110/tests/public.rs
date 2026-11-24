use task_110::*;

#[test]
fn public_examples() {
    assert_eq!(ticket_text_10(65535, 4), "B-10/4");
    assert_eq!(ticket_text_10(13, 14), "B-10/13");
}
