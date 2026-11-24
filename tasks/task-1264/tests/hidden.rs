use task_1264::*;

#[test]
fn edge_cases() {
    assert_eq!(positional_options_peaks("-q -q -- -v"), vec![]);
    assert_eq!(
        positional_options_peaks("--level=3 data --level=9"),
        vec![4u32]
    );
    assert_eq!(positional_options_peaks("a --bad --level=x b"), vec![1u32]);
    assert_eq!(
        positional_options_peaks("input -v -q input --level=255"),
        vec![5u32]
    );
    assert_eq!(positional_options_peaks("-- --level=4 x"), vec![1u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1264::TRACK_PROOF, 0x52540000 | 1264);
}
