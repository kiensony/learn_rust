use task_2244::*;

#[test]
fn edge_cases() {
    assert_eq!(
        borrowed_windows_peaks("a\nblue\nblue\r\nz\n"),
        vec![24842u32, 25196u32, 27765u32, 30053u32, 31242u32]
    );
    assert_eq!(
        borrowed_windows_peaks("   x\té\n\ncat\n"),
        vec![8224u32, 8312u32, 30729u32, 50089u32]
    );
    assert_eq!(
        borrowed_windows_peaks("nine\nq\nthree\nx\n"),
        vec![28265u32, 28938u32, 29800u32, 30730u32]
    );
    assert_eq!(
        borrowed_windows_peaks("a b c\n\r\nup\tdown"),
        vec![24864u32, 25120u32, 25354u32, 30064u32, 30574u32]
    );
    assert_eq!(
        borrowed_windows_peaks("\n\nzz\n"),
        vec![2570u32, 2682u32, 31354u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2244::TRACK_PROOF, 0x52540000 | 2244);
}
