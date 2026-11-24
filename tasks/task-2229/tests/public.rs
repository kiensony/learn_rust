use task_2229::*;

#[test]
fn public_examples() {
    assert_eq!(borrowed_scalars_longest_run(""), 0);
    assert_eq!(borrowed_scalars_longest_run("a"), 1);
}
