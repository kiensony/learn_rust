use task_1290::*;

#[test]
fn edge_cases() {
    assert_eq!(option_terminator_reverse("-q -q -- -v"), vec![2u32]);
    assert_eq!(
        option_terminator_reverse("--level=3 data --level=9"),
        vec![]
    );
    assert_eq!(option_terminator_reverse("a --bad --level=x b"), vec![]);
    assert_eq!(
        option_terminator_reverse("input -v -q input --level=255"),
        vec![]
    );
    assert_eq!(
        option_terminator_reverse("-- --level=4 x"),
        vec![1u32, 9u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1290::TRACK_PROOF, 0x52540000 | 1290);
}
