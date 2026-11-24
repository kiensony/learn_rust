use task_192::*;

#[test]
fn edge_cases() {
    assert_eq!(comfort_zone_8(13, 1), false);
    assert_eq!(comfort_zone_8(0, 2), false);
    assert_eq!(comfort_zone_8(1, 3), false);
    assert_eq!(comfort_zone_8(7, 4), false);
    assert_eq!(comfort_zone_8(8, 5), false);
    assert_eq!(comfort_zone_8(9, 6), false);
    assert_eq!(comfort_zone_8(65535, 0), false);
    assert_eq!(comfort_zone_8(11, 2), true);
    assert_eq!(comfort_zone_8(16, 3), false);
    assert_eq!(comfort_zone_8(17, 4), false);
    assert_eq!(comfort_zone_8(18, 3), false);
    assert_eq!(comfort_zone_8(3, 18), false);
    assert_eq!(comfort_zone_8(4, 4), false);
    assert_eq!(comfort_zone_8(0, 0), false);
    assert_eq!(comfort_zone_8(1, 0), false);
    assert_eq!(comfort_zone_8(7, 0), false);
    assert_eq!(comfort_zone_8(8, 0), true);
    assert_eq!(comfort_zone_8(9, 0), true);
    assert_eq!(comfort_zone_8(10, 0), true);
    assert_eq!(comfort_zone_8(11, 0), false);
    assert_eq!(comfort_zone_8(16, 0), false);
    assert_eq!(comfort_zone_8(17, 0), false);
    assert_eq!(comfort_zone_8(20, 0), false);
    assert_eq!(comfort_zone_8(100, 0), false);
}

#[test]
fn submission_marker() {
    assert_eq!(task_192::TRACK_PROOF, 0x52540000 | 192);
}
