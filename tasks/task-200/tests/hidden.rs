use task_200::*;

#[test]
fn edge_cases() {
    assert_eq!(admission_gate_10(65535, 0), false);
    assert_eq!(admission_gate_10(1, 2), false);
    assert_eq!(admission_gate_10(65535, 9), false);
    assert_eq!(admission_gate_10(10, 11), true);
    assert_eq!(admission_gate_10(65535, 11), true);
    assert_eq!(admission_gate_10(65535, 65535), true);
    assert_eq!(admission_gate_10(65535, 12), true);
    assert_eq!(admission_gate_10(65535, 20), true);
    assert_eq!(admission_gate_10(21, 22), true);
    assert_eq!(admission_gate_10(1, 0), false);
    assert_eq!(admission_gate_10(0, 1), false);
    assert_eq!(admission_gate_10(0, 0), false);
    assert_eq!(admission_gate_10(1, 1), false);
    assert_eq!(admission_gate_10(9, 9), false);
    assert_eq!(admission_gate_10(10, 10), true);
    assert_eq!(admission_gate_10(11, 11), true);
    assert_eq!(admission_gate_10(12, 12), true);
    assert_eq!(admission_gate_10(13, 13), true);
    assert_eq!(admission_gate_10(20, 20), true);
    assert_eq!(admission_gate_10(21, 21), true);
    assert_eq!(admission_gate_10(100, 100), true);
}

#[test]
fn submission_marker() {
    assert_eq!(task_200::TRACK_PROOF, 0x52540000 | 200);
}
