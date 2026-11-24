use task_2243::*;

#[test]
fn edge_cases() {
    assert_eq!(
        borrowed_windows_bounds("a\nblue\nblue\r\nz\n"),
        Some((2658u32, 31242u32))
    );
    assert_eq!(
        borrowed_windows_bounds("   x\té\n\ncat\n"),
        Some((2499u32, 50089u32))
    );
    assert_eq!(
        borrowed_windows_bounds("nine\nq\nthree\nx\n"),
        Some((2673u32, 30730u32))
    );
    assert_eq!(
        borrowed_windows_bounds("a b c\n\r\nup\tdown"),
        Some((2404u32, 30574u32))
    );
    assert_eq!(
        borrowed_windows_bounds("\n\nzz\n"),
        Some((2570u32, 31354u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2243::TRACK_PROOF, 0x52540000 | 2243);
}
