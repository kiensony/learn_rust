use task_2403::*;

#[test]
fn boundaries() {
    assert_eq!(encode(Tag::Data), 1);
    assert_eq!(encode(Tag::End), 255);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 2403);
}
