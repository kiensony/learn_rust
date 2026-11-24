use task_318::*;

#[test]
fn public_examples() {
    assert_eq!(unpack((String::from("tea"), 3)), (3, String::from("tea")));
    assert_eq!(unpack((String::from("ink"), 21)), (21, String::from("ink")));
}
