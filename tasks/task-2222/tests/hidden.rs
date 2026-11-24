use task_2222::*;

#[test]
fn edge_cases() {
    assert_eq!(borrowed_scalars_total("a\nblue\nblue\r\nz\n"), 105);
    assert_eq!(borrowed_scalars_total("   x\té\n\ncat\n"), 72);
    assert_eq!(borrowed_scalars_total("nine\nq\nthree\nx\n"), 105);
    assert_eq!(borrowed_scalars_total("a b c\n\r\nup\tdown"), 105);
    assert_eq!(borrowed_scalars_total("\n\nzz\n"), 10);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2222::TRACK_PROOF, 0x52540000 | 2222);
}
