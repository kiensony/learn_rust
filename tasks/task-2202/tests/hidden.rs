use task_2202::*;

#[test]
fn edge_cases() {
    assert_eq!(borrowed_prefixes_total("a\nblue\nblue\r\nz\n"), 10);
    assert_eq!(borrowed_prefixes_total("   x\té\n\ncat\n"), 6);
    assert_eq!(borrowed_prefixes_total("nine\nq\nthree\nx\n"), 11);
    assert_eq!(borrowed_prefixes_total("a b c\n\r\nup\tdown"), 9);
    assert_eq!(borrowed_prefixes_total("\n\nzz\n"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2202::TRACK_PROOF, 0x52540000 | 2202);
}
