use task_2837::*;

#[test]
fn edge_cases() {
    assert_eq!(
        service_expansion_distinct(&[9, 2, 2, 8, 0, 5]),
        vec![0u32, 4u32, 10u32, 16u32, 18u32]
    );
    assert_eq!(service_expansion_distinct(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        service_expansion_distinct(&[255, 1, 255, 2]),
        vec![2u32, 4u32, 510u32]
    );
    assert_eq!(
        service_expansion_distinct(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            0u32, 6u32, 12u32, 16u32, 18u32, 24u32, 30u32, 42u32, 52u32, 62u32
        ]
    );
    assert_eq!(
        service_expansion_distinct(&[17, 17, 1, 0, 2]),
        vec![0u32, 2u32, 4u32, 34u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2837::TRACK_PROOF, 0x52540000 | 2837);
}
