use task_2226::*;

#[test]
fn edge_cases() {
    assert_eq!(borrowed_scalars_descending_steps("a\nblue\nblue\r\nz\n"), 0);
    assert_eq!(borrowed_scalars_descending_steps("   x\té\n\ncat\n"), 0);
    assert_eq!(borrowed_scalars_descending_steps("nine\nq\nthree\nx\n"), 0);
    assert_eq!(borrowed_scalars_descending_steps("a b c\n\r\nup\tdown"), 0);
    assert_eq!(borrowed_scalars_descending_steps("\n\nzz\n"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2226::TRACK_PROOF, 0x52540000 | 2226);
}
