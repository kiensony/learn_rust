use task_2239::*;

#[test]
fn edge_cases() {
    assert_eq!(zero_copy_lines_longest_run("a\nblue\nblue\r\nz\n"), 2);
    assert_eq!(zero_copy_lines_longest_run("   x\té\n\ncat\n"), 1);
    assert_eq!(zero_copy_lines_longest_run("nine\nq\nthree\nx\n"), 1);
    assert_eq!(zero_copy_lines_longest_run("a b c\n\r\nup\tdown"), 1);
    assert_eq!(zero_copy_lines_longest_run("\n\nzz\n"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2239::TRACK_PROOF, 0x52540000 | 2239);
}
