use task_2663::*;

#[test]
fn edge_cases() {
    assert_eq!(
        lending_first_byte_bounds(&[9, 2, 2, 8, 0, 5]),
        Some((8u32, 9u32))
    );
    assert_eq!(lending_first_byte_bounds(&[0, 0, 0]), Some((0u32, 0u32)));
    assert_eq!(
        lending_first_byte_bounds(&[255, 1, 255, 2]),
        Some((2u32, 255u32))
    );
    assert_eq!(
        lending_first_byte_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        Some((0u32, 31u32))
    );
    assert_eq!(
        lending_first_byte_bounds(&[17, 17, 1, 0, 2]),
        Some((0u32, 17u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2663::TRACK_PROOF, 0x52540000 | 2663);
}
