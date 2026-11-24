use task_1012::*;

#[test]
fn public_examples() {
    assert_eq!(swap_pair((1, 4)), (4, 1));
    assert_eq!(swap_pair(("north", "west")), ("west", "north"));
}
