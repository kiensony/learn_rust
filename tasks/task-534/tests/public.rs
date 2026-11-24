use task_534::*;

#[test]
fn public_examples() {
    assert_eq!(preview("rust", 2), "ru");
    assert_eq!(preview("葉ab", 2), "");
}
