use task_2202::*;

#[test]
fn public_examples() {
    assert_eq!(borrowed_prefixes_total(""), 0);
    assert_eq!(borrowed_prefixes_total("a"), 1);
}
