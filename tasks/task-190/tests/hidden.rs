use task_190::*;

#[test]
fn edge_cases() {
    assert_eq!(admission_gate_8(0, 2), false);
    assert_eq!(admission_gate_8(1, 3), false);
    assert_eq!(admission_gate_8(7, 4), false);
    assert_eq!(admission_gate_8(8, 5), false);
    assert_eq!(admission_gate_8(9, 6), false);
    assert_eq!(admission_gate_8(65535, 0), true);
    assert_eq!(admission_gate_8(10, 1), true);
    assert_eq!(admission_gate_8(11, 2), true);
    assert_eq!(admission_gate_8(16, 3), true);
    assert_eq!(admission_gate_8(17, 4), true);
    assert_eq!(admission_gate_8(18, 3), true);
    assert_eq!(admission_gate_8(3, 18), true);
    assert_eq!(admission_gate_8(4, 4), false);
    assert_eq!(admission_gate_8(0, 0), false);
    assert_eq!(admission_gate_8(1, 0), false);
    assert_eq!(admission_gate_8(7, 0), false);
    assert_eq!(admission_gate_8(8, 0), true);
    assert_eq!(admission_gate_8(9, 0), true);
    assert_eq!(admission_gate_8(10, 0), true);
    assert_eq!(admission_gate_8(11, 0), true);
    assert_eq!(admission_gate_8(16, 0), true);
    assert_eq!(admission_gate_8(17, 0), true);
    assert_eq!(admission_gate_8(20, 0), true);
    assert_eq!(admission_gate_8(100, 0), true);
}

#[test]
fn submission_marker() {
    assert_eq!(task_190::TRACK_PROOF, 0x52540000 | 190);
}
