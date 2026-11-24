use task_1272::*;

#[test]
fn edge_cases() {
    assert_eq!(explicit_levels_total("-q -q -- -v"), 0);
    assert_eq!(explicit_levels_total("--level=3 data --level=9"), 12);
    assert_eq!(explicit_levels_total("a --bad --level=x b"), 0);
    assert_eq!(explicit_levels_total("input -v -q input --level=255"), 255);
    assert_eq!(explicit_levels_total("-- --level=4 x"), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1272::TRACK_PROOF, 0x52540000 | 1272);
}
