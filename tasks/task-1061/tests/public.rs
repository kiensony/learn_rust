use task_1061::*;

#[test]
fn public_examples() {
    assert_eq!(key_of(&Record(4u8, 9u8)), &4);
    assert_eq!(key_of(&Record("A", "B")), &"A");
}
