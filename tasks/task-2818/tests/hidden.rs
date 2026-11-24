use task_2818::*;

#[test]
fn edge_cases() {
    assert_eq!(
        service_validation_steps(&[9, 2, 2, 8, 0, 5]),
        vec![0u32, 0u32, 0u32, 1u32, 1u32]
    );
    assert_eq!(service_validation_steps(&[0, 0, 0]), vec![0u32, 0u32]);
    assert_eq!(
        service_validation_steps(&[255, 1, 255, 2]),
        vec![0u32, 0u32, 0u32]
    );
    assert_eq!(
        service_validation_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 1u32, 0u32, 0u32]
    );
    assert_eq!(
        service_validation_steps(&[17, 17, 1, 0, 2]),
        vec![0u32, 0u32, 1u32, 1u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2818::TRACK_PROOF, 0x52540000 | 2818);
}
