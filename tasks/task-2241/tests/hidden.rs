use task_2241::*;

#[test]
fn edge_cases() {
    assert_eq!(
        borrowed_windows_trace("a\nblue\nblue\r\nz\n"),
        vec![
            24842u32, 2658u32, 25196u32, 27765u32, 30053u32, 25866u32, 2658u32, 25196u32, 27765u32,
            30053u32, 25869u32, 3338u32, 2682u32, 31242u32
        ]
    );
    assert_eq!(
        borrowed_windows_trace("   x\té\n\ncat\n"),
        vec![
            8224u32, 8224u32, 8312u32, 30729u32, 2499u32, 50089u32, 43274u32, 2570u32, 2659u32,
            25441u32, 24948u32, 29706u32
        ]
    );
    assert_eq!(
        borrowed_windows_trace("nine\nq\nthree\nx\n"),
        vec![
            28265u32, 26990u32, 28261u32, 25866u32, 2673u32, 28938u32, 2676u32, 29800u32, 26738u32,
            29285u32, 25957u32, 25866u32, 2680u32, 30730u32
        ]
    );
    assert_eq!(
        borrowed_windows_trace("a b c\n\r\nup\tdown"),
        vec![
            24864u32, 8290u32, 25120u32, 8291u32, 25354u32, 2573u32, 3338u32, 2677u32, 30064u32,
            28681u32, 2404u32, 25711u32, 28535u32, 30574u32
        ]
    );
    assert_eq!(
        borrowed_windows_trace("\n\nzz\n"),
        vec![2570u32, 2682u32, 31354u32, 31242u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2241::TRACK_PROOF, 0x52540000 | 2241);
}
