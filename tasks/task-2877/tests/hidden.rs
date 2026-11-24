use task_2877::*;

#[test]
fn edge_cases() {
    assert_eq!(unpack(0x7fff), None);
    assert_eq!(unpack(0xffff), Some(4095));
    assert_eq!(unpack(0xb000), Some(0));
}

#[test]
fn submission_marker() {
    assert_eq!(task_2877::TRACK_PROOF, 0x52540000 | 2877);
}
