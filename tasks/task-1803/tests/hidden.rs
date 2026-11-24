use task_1803::*;

#[test]
fn boundaries() {
    assert_eq!(worker(0), 1);
    assert_eq!(worker(65535), 65535);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 1803);
}
