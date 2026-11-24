use task_1263::*;

#[test]
fn edge_cases() {
    assert_eq!(positional_options_bounds("-q -q -- -v"), None);
    assert_eq!(
        positional_options_bounds("--level=3 data --level=9"),
        Some((4u32, 4u32))
    );
    assert_eq!(
        positional_options_bounds("a --bad --level=x b"),
        Some((1u32, 1u32))
    );
    assert_eq!(
        positional_options_bounds("input -v -q input --level=255"),
        Some((5u32, 5u32))
    );
    assert_eq!(
        positional_options_bounds("-- --level=4 x"),
        Some((1u32, 1u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1263::TRACK_PROOF, 0x52540000 | 1263);
}
