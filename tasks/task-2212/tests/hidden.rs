use task_2212::*;

#[test]
fn edge_cases() {
    assert_eq!(borrowed_offsets_total("a\nblue\nblue\r\nz\n"), 33);
    assert_eq!(borrowed_offsets_total("   x\té\n\ncat\n"), 27);
    assert_eq!(borrowed_offsets_total("nine\nq\nthree\nx\n"), 36);
    assert_eq!(borrowed_offsets_total("a b c\n\r\nup\tdown"), 12);
    assert_eq!(borrowed_offsets_total("\n\nzz\n"), 5);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2212::TRACK_PROOF, 0x52540000 | 2212);
}
