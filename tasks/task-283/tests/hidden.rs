use task_283::*;

#[test]
fn edge_cases() {
    assert_eq!(double_a_symbol_tilde(""), "");
    assert_eq!(double_a_symbol_tilde("~"), "~~");
    assert_eq!(double_a_symbol_tilde("~~"), "~~~~");
    assert_eq!(double_a_symbol_tilde(" café "), " café ");
    assert_eq!(double_a_symbol_tilde("a~b"), "a~~b");
    assert_eq!(double_a_symbol_tilde("🐈~猫"), "🐈~~猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_283::TRACK_PROOF, 0x52540000 | 283);
}
