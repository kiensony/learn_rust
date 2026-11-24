use task_1489::*;

#[test]
fn public_examples() {
    assert_eq!(response_boundaries_longest_run(""), 0);
    assert_eq!(response_boundaries_longest_run("200"), 1);
}
