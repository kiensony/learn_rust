use task_202::*;

#[test]
fn edge_cases() {
    assert_eq!(comfort_zone_10(13, 14), false);
    assert_eq!(comfort_zone_10(65535, 0), false);
    assert_eq!(comfort_zone_10(1, 2), false);
    assert_eq!(comfort_zone_10(65535, 9), false);
    assert_eq!(comfort_zone_10(65535, 11), true);
    assert_eq!(comfort_zone_10(65535, 65535), false);
    assert_eq!(comfort_zone_10(65535, 12), true);
    assert_eq!(comfort_zone_10(65535, 20), false);
    assert_eq!(comfort_zone_10(21, 22), false);
    assert_eq!(comfort_zone_10(1, 0), false);
    assert_eq!(comfort_zone_10(0, 1), false);
    assert_eq!(comfort_zone_10(0, 0), false);
    assert_eq!(comfort_zone_10(1, 1), false);
    assert_eq!(comfort_zone_10(9, 9), false);
    assert_eq!(comfort_zone_10(10, 10), true);
    assert_eq!(comfort_zone_10(11, 11), true);
    assert_eq!(comfort_zone_10(12, 12), true);
    assert_eq!(comfort_zone_10(13, 13), false);
    assert_eq!(comfort_zone_10(20, 20), false);
    assert_eq!(comfort_zone_10(21, 21), false);
    assert_eq!(comfort_zone_10(100, 100), false);
}

#[test]
fn submission_marker() {
    assert_eq!(task_202::TRACK_PROOF, 0x52540000 | 202);
}
