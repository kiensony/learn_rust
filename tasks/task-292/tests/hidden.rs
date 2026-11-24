use task_292::*;

#[test]
fn edge_cases() {
    assert_eq!(boundary_trim_plus(""), "");
    assert_eq!(boundary_trim_plus("+"), "");
    assert_eq!(boundary_trim_plus("++"), "");
    assert_eq!(boundary_trim_plus(" café "), " café ");
    assert_eq!(boundary_trim_plus("a+b"), "a+b");
    assert_eq!(boundary_trim_plus("🐈+猫"), "🐈+猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_292::TRACK_PROOF, 0x52540000 | 292);
}
