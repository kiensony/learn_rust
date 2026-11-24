use task_1268::*;

#[test]
fn edge_cases() {
    assert_eq!(positional_options_steps("-q -q -- -v"), vec![]);
    assert_eq!(positional_options_steps("--level=3 data --level=9"), vec![]);
    assert_eq!(positional_options_steps("a --bad --level=x b"), vec![0u32]);
    assert_eq!(
        positional_options_steps("input -v -q input --level=255"),
        vec![0u32]
    );
    assert_eq!(positional_options_steps("-- --level=4 x"), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1268::TRACK_PROOF, 0x52540000 | 1268);
}
