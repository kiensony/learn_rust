use task_282::*;

#[test]
fn edge_cases() {
    assert_eq!(boundary_trim_tilde(""), "");
    assert_eq!(boundary_trim_tilde("~"), "");
    assert_eq!(boundary_trim_tilde("~~"), "");
    assert_eq!(boundary_trim_tilde(" café "), " café ");
    assert_eq!(boundary_trim_tilde("a~b"), "a~b");
    assert_eq!(boundary_trim_tilde("🐈~猫"), "🐈~猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_282::TRACK_PROOF, 0x52540000 | 282);
}
