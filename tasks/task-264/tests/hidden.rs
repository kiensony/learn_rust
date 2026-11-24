use task_264::*;

#[test]
fn edge_cases() {
    assert_eq!(double_a_symbol_hash(""), "");
    assert_eq!(double_a_symbol_hash("#"), "##");
    assert_eq!(double_a_symbol_hash("##"), "####");
    assert_eq!(double_a_symbol_hash(" café "), " café ");
    assert_eq!(double_a_symbol_hash("a#b"), "a##b");
    assert_eq!(double_a_symbol_hash("🐈#猫"), "🐈##猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_264::TRACK_PROOF, 0x52540000 | 264);
}
