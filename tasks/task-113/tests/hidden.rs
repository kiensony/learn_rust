use task_113::*;

#[test]
fn boundaries() {
    assert_eq!(active(0), false);
    assert_eq!(active(2), true);
    assert_eq!(active(255), true);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 113);
}
