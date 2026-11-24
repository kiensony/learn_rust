use task_905::*;

#[test]
fn boundaries() {
    assert_eq!(quote(|_, b| b, 30, 0), 0);
    assert_eq!(quote(|a, b| a + b, 1, 2), 3);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 905);
}
