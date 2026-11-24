use task_2838::*;

#[test]
fn edge_cases() {
    assert_eq!(
        service_expansion_steps(&[9, 2, 2, 8, 0, 5]),
        vec![14u32, 0u32, 12u32, 16u32, 10u32]
    );
    assert_eq!(service_expansion_steps(&[0, 0, 0]), vec![0u32, 0u32]);
    assert_eq!(
        service_expansion_steps(&[255, 1, 255, 2]),
        vec![508u32, 508u32, 506u32]
    );
    assert_eq!(
        service_expansion_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![6u32, 6u32, 6u32, 18u32, 12u32, 30u32, 52u32, 36u32, 46u32]
    );
    assert_eq!(
        service_expansion_steps(&[17, 17, 1, 0, 2]),
        vec![0u32, 32u32, 2u32, 4u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2838::TRACK_PROOF, 0x52540000 | 2838);
}
