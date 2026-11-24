use task_2209::*;

#[test]
fn edge_cases() {
    assert_eq!(borrowed_prefixes_longest_run("a\nblue\nblue\r\nz\n"), 2);
    assert_eq!(borrowed_prefixes_longest_run("   x\té\n\ncat\n"), 1);
    assert_eq!(borrowed_prefixes_longest_run("nine\nq\nthree\nx\n"), 1);
    assert_eq!(borrowed_prefixes_longest_run("a b c\n\r\nup\tdown"), 3);
    assert_eq!(borrowed_prefixes_longest_run("\n\nzz\n"), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2209::TRACK_PROOF, 0x52540000 | 2209);
}
