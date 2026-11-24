use task_1599::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_lanes_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        if cfg!(feature = "wide") {
            vec![48u32, 96u32, 144u32]
        } else {
            vec![3u32, 6u32, 9u32]
        }
    );
    assert_eq!(feature_lanes_frequent_readings(&[], 0), vec![]);
}
