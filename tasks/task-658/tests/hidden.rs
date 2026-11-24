use task_658::*;

#[test]
fn boundaries() {
    assert_eq!(instruction(Light::Red), "wait");
    assert_eq!(instruction(Light::Amber), "ready");
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 658);
}
