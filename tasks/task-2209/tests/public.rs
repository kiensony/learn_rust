use task_2209::*;

#[test]
fn public_examples() {
    assert_eq!(borrowed_prefixes_longest_run(""), 0);
    assert_eq!(borrowed_prefixes_longest_run("a"), 1);
}
