use task_516::*;

#[test]
fn boundaries() {
    assert_eq!(bars(""), 0);
    assert_eq!(bars("雪||☀"), 2);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 516);
}
