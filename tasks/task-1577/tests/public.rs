use task_1577::*;

#[test]
fn public_examples() {
    assert_eq!(native_archive("io"), "track_io");
    assert_eq!(native_archive("metrics-v1"), "track_metrics_v1");
}
