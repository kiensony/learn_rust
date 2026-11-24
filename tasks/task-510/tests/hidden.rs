use task_510::*;

#[test]
fn boundaries() {
    assert_eq!(is_tab(' '), false);
    assert_eq!(is_tab('雪'), false);
    assert_eq!(is_tab('\n'), false);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 510);
}
