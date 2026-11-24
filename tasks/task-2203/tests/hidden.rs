use task_2203::*;

#[test]
fn edge_cases() {
    assert_eq!(
        borrowed_prefixes_bounds("a\nblue\nblue\r\nz\n"),
        Some((1u32, 4u32))
    );
    assert_eq!(
        borrowed_prefixes_bounds("   x\té\n\ncat\n"),
        Some((1u32, 3u32))
    );
    assert_eq!(
        borrowed_prefixes_bounds("nine\nq\nthree\nx\n"),
        Some((1u32, 5u32))
    );
    assert_eq!(
        borrowed_prefixes_bounds("a b c\n\r\nup\tdown"),
        Some((1u32, 4u32))
    );
    assert_eq!(borrowed_prefixes_bounds("\n\nzz\n"), Some((2u32, 2u32)));
}

#[test]
fn submission_marker() {
    assert_eq!(task_2203::TRACK_PROOF, 0x52540000 | 2203);
}
