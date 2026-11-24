use task_267::*;

#[test]
fn boundaries() {
    assert_eq!(badge_bytes("Ada"), 3);
    assert_eq!(badge_bytes("雪"), 3);
    assert_eq!(badge_bytes(" "), 1);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 267);
}
