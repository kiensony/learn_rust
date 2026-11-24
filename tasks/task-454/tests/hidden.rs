use task_454::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut a = [0, 0];
            let moved = transfer(&mut a, 0);
            (a, moved)
        },
        ([0, 0], 0)
    );
    assert_eq!(
        {
            let mut a = [65535, 65535];
            let moved = transfer(&mut a, 99);
            (a, moved)
        },
        ([65535, 65535], 0)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_454::TRACK_PROOF, 0x52540000 | 454);
}
