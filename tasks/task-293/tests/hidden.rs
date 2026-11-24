use task_293::*;

#[test]
fn edge_cases() {
    assert_eq!(double_a_symbol_plus(""), "");
    assert_eq!(double_a_symbol_plus("+"), "++");
    assert_eq!(double_a_symbol_plus("++"), "++++");
    assert_eq!(double_a_symbol_plus(" café "), " café ");
    assert_eq!(double_a_symbol_plus("a+b"), "a++b");
    assert_eq!(double_a_symbol_plus("🐈+猫"), "🐈++猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_293::TRACK_PROOF, 0x52540000 | 293);
}
