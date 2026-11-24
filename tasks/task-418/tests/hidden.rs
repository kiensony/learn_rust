use task_418::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = [];
            let changed = cap(&mut data, 0);
            (data, changed)
        },
        ([], 0)
    );
    assert_eq!(
        {
            let mut data = [65535];
            let changed = cap(&mut data, 0);
            (data, changed)
        },
        ([0], 1)
    );
    assert_eq!(
        {
            let mut data = [0, 1];
            let changed = cap(&mut data, 0);
            (data, changed)
        },
        ([0, 0], 1)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_418::TRACK_PROOF, 0x52540000 | 418);
}
