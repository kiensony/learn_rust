use task_291::*;

#[test]
fn edge_cases() {
    assert_eq!(remove_a_suffix_plus(""), "");
    assert_eq!(remove_a_suffix_plus("+"), "");
    assert_eq!(remove_a_suffix_plus("++"), "+");
    assert_eq!(remove_a_suffix_plus(" café "), " café ");
    assert_eq!(remove_a_suffix_plus("a+b"), "a+b");
    assert_eq!(remove_a_suffix_plus("🐈+猫"), "🐈+猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_291::TRACK_PROOF, 0x52540000 | 291);
}
