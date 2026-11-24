use task_316::*;

#[test]
fn boundaries() {
    assert_eq!(release("".into()), 0);
    assert_eq!(release("Δ".into()), 2);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 316);
}
