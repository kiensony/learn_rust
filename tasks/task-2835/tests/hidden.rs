use task_2835::*;

#[test]
fn edge_cases() {
    assert_eq!(
        service_expansion_changes(&[9, 2, 2, 8, 0, 5]),
        vec![18u32, 4u32, 16u32, 0u32, 10u32]
    );
    assert_eq!(service_expansion_changes(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        service_expansion_changes(&[255, 1, 255, 2]),
        vec![510u32, 2u32, 510u32, 4u32]
    );
    assert_eq!(
        service_expansion_changes(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            6u32, 12u32, 18u32, 24u32, 42u32, 30u32, 0u32, 52u32, 16u32, 62u32
        ]
    );
    assert_eq!(
        service_expansion_changes(&[17, 17, 1, 0, 2]),
        vec![34u32, 2u32, 0u32, 4u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2835::TRACK_PROOF, 0x52540000 | 2835);
}
