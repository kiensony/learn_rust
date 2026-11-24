use task_296::*;

#[test]
fn public_examples() {
    assert_eq!(blank_notice_plus("cat"), "cat");
    assert_eq!(blank_notice_plus("+hi+"), "+hi+");
}
