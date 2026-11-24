use task_286::*;

#[test]
fn public_examples() {
    assert_eq!(blank_notice_tilde("cat"), "cat");
    assert_eq!(blank_notice_tilde("~hi~"), "~hi~");
}
