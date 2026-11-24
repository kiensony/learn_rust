use task_1622::*;

#[test]
fn edge_cases() {
    assert_eq!(round_trip_property_total(&[9, 2, 2, 8, 0, 5]), 0);
    assert_eq!(round_trip_property_total(&[0, 0, 0]), 0);
    assert_eq!(round_trip_property_total(&[255, 1, 255, 2]), 2);
    assert_eq!(
        round_trip_property_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        0
    );
    assert_eq!(round_trip_property_total(&[17, 17, 1, 0, 2]), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1622::TRACK_PROOF, 0x52540000 | 1622);
}
