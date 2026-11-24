use task_1262::*;

#[test]
fn edge_cases() {
    assert_eq!(positional_options_total("-q -q -- -v"), 0);
    assert_eq!(positional_options_total("--level=3 data --level=9"), 4);
    assert_eq!(positional_options_total("a --bad --level=x b"), 2);
    assert_eq!(
        positional_options_total("input -v -q input --level=255"),
        10
    );
    assert_eq!(positional_options_total("-- --level=4 x"), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1262::TRACK_PROOF, 0x52540000 | 1262);
}
