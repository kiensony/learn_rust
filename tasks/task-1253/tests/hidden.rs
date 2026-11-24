use task_1253::*;

#[test]
fn edge_cases() {
    assert_eq!(command_arguments_bounds("-q -q -- -v"), Some((2u32, 2u32)));
    assert_eq!(
        command_arguments_bounds("--level=3 data --level=9"),
        Some((4u32, 9u32))
    );
    assert_eq!(
        command_arguments_bounds("a --bad --level=x b"),
        Some((1u32, 9u32))
    );
    assert_eq!(
        command_arguments_bounds("input -v -q input --level=255"),
        Some((2u32, 11u32))
    );
    assert_eq!(
        command_arguments_bounds("-- --level=4 x"),
        Some((1u32, 9u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1253::TRACK_PROOF, 0x52540000 | 1253);
}
