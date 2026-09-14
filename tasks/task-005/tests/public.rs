use task_005::longest;

#[test]
fn picks_longer() {
    assert_eq!(longest("aa", "b"), "aa");
    assert_eq!(longest("a", "bb"), "bb");
}
