use task_368::*;

#[test]
fn edge_cases() {
    assert_eq!(label_info(&String::from("")), (0, true));
    assert_eq!(label_info(&String::from("λ")), (2, false));
    assert_eq!(label_info(&String::from("  x  ")), (5, true));
}

#[test]
fn submission_marker() {
    assert_eq!(task_368::TRACK_PROOF, 0x52540000 | 368);
}
