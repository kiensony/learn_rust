use task_087::*;

#[test]
fn edge_cases() {
    assert_eq!(after_delivery(0, 0), 0);
    assert_eq!(after_delivery(-32768, -32768), -65536);
    assert_eq!(after_delivery(32767, 32767), 65534);
}

#[test]
fn submission_marker() {
    assert_eq!(task_087::TRACK_PROOF, 0x52540000 | 87);
}
