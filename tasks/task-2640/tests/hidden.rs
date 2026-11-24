use task_2640::*;

#[test]
fn edge_cases() {
    assert_eq!(
        embedded_saturation_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![10u32, 0u32, 16u32, 4u32, 4u32, 18u32]
    );
    assert_eq!(
        embedded_saturation_reverse(&[0, 0, 0]),
        vec![0u32, 0u32, 0u32]
    );
    assert_eq!(
        embedded_saturation_reverse(&[255, 1, 255, 2]),
        vec![4u32, 255u32, 2u32, 255u32]
    );
    assert_eq!(
        embedded_saturation_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            62u32, 16u32, 52u32, 0u32, 30u32, 42u32, 24u32, 18u32, 12u32, 6u32
        ]
    );
    assert_eq!(
        embedded_saturation_reverse(&[17, 17, 1, 0, 2]),
        vec![4u32, 0u32, 2u32, 34u32, 34u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2640::TRACK_PROOF, 0x52540000 | 2640);
}
