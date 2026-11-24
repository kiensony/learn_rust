use task_1288::*;

#[test]
fn edge_cases() {
    assert_eq!(option_terminator_steps("-q -q -- -v"), vec![]);
    assert_eq!(option_terminator_steps("--level=3 data --level=9"), vec![]);
    assert_eq!(option_terminator_steps("a --bad --level=x b"), vec![]);
    assert_eq!(
        option_terminator_steps("input -v -q input --level=255"),
        vec![]
    );
    assert_eq!(option_terminator_steps("-- --level=4 x"), vec![8u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1288::TRACK_PROOF, 0x52540000 | 1288);
}
