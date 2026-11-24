use task_303::*;

#[test]
fn edge_cases() {
    assert_eq!(double_a_symbol_bang(""), "");
    assert_eq!(double_a_symbol_bang("!"), "!!");
    assert_eq!(double_a_symbol_bang("!!"), "!!!!");
    assert_eq!(double_a_symbol_bang(" café "), " café ");
    assert_eq!(double_a_symbol_bang("a!b"), "a!!b");
    assert_eq!(double_a_symbol_bang("🐈!猫"), "🐈!!猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_303::TRACK_PROOF, 0x52540000 | 303);
}
