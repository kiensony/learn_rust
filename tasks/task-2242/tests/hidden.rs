use task_2242::*;

#[test]
fn edge_cases() {
    assert_eq!(borrowed_windows_total("a\nblue\nblue\r\nz\n"), 285183);
    assert_eq!(borrowed_windows_total("   x\té\n\ncat\n"), 236675);
    assert_eq!(borrowed_windows_total("nine\nq\nthree\nx\n"), 314725);
    assert_eq!(borrowed_windows_total("a b c\n\r\nup\tdown"), 246476);
    assert_eq!(borrowed_windows_total("\n\nzz\n"), 67848);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2242::TRACK_PROOF, 0x52540000 | 2242);
}
