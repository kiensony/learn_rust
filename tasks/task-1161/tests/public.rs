use task_1161::*;

#[test]
fn public_examples() {
    assert_eq!(setting("host=local"), Some(("host", "local")));
    assert_eq!(setting("a=b=c"), Some(("a", "b=c")));
}
