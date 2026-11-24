use task_2877::*;

#[test]
fn public_examples() {
    assert_eq!(unpack(0), None);
    assert_eq!(unpack(0x8005), Some(5));
}
