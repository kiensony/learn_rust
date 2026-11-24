use task_273::*;

#[test]
fn edge_cases() {
    assert_eq!(double_a_symbol_star(""), "");
    assert_eq!(double_a_symbol_star("*"), "**");
    assert_eq!(double_a_symbol_star("**"), "****");
    assert_eq!(double_a_symbol_star(" café "), " café ");
    assert_eq!(double_a_symbol_star("a*b"), "a**b");
    assert_eq!(double_a_symbol_star("🐈*猫"), "🐈**猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_273::TRACK_PROOF, 0x52540000 | 273);
}
