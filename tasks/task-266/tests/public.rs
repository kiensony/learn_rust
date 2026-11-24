use task_266::*;

#[test]
fn public_examples() {
    assert_eq!(blank_notice_hash("cat"), "cat");
    assert_eq!(blank_notice_hash("#hi#"), "#hi#");
}
