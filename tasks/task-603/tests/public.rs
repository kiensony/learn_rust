use task_603::*;

#[test]
fn public_examples() {
    assert_eq!(choose(true, "stop", "go"), "stop");
    assert_eq!(choose(false, "hold", "walk"), "walk");
}
