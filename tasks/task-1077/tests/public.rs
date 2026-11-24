use task_1077::*;

#[test]
fn public_examples() {
    assert_eq!(consume_first(vec![3, 5].into_iter()), Some(3));
    assert_eq!(consume_first(["α", "β"].into_iter()), Some("α"));
}
