use task_294::*;

#[test]
fn public_examples() {
    assert_eq!(read_a_heading_plus("cat"), "cat");
    assert_eq!(read_a_heading_plus("+hi+"), "");
}
