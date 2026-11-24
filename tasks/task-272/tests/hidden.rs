use task_272::*;

#[test]
fn edge_cases() {
    assert_eq!(boundary_trim_star(""), "");
    assert_eq!(boundary_trim_star("*"), "");
    assert_eq!(boundary_trim_star("**"), "");
    assert_eq!(boundary_trim_star(" café "), " café ");
    assert_eq!(boundary_trim_star("a*b"), "a*b");
    assert_eq!(boundary_trim_star("🐈*猫"), "🐈*猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_272::TRACK_PROOF, 0x52540000 | 272);
}
