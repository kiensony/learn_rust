use task_263::*;

#[test]
fn edge_cases() {
    assert_eq!(boundary_trim_hash(""), "");
    assert_eq!(boundary_trim_hash("#"), "");
    assert_eq!(boundary_trim_hash("##"), "");
    assert_eq!(boundary_trim_hash(" café "), " café ");
    assert_eq!(boundary_trim_hash("a#b"), "a#b");
    assert_eq!(boundary_trim_hash("🐈#猫"), "🐈#猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_263::TRACK_PROOF, 0x52540000 | 263);
}
