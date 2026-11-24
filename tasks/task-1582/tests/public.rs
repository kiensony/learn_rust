use task_1582::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_lanes_total(&[]),
        if cfg!(feature = "wide") { 0 } else { 0 }
    );
    assert_eq!(
        feature_lanes_total(&[4]),
        if cfg!(feature = "wide") { 64 } else { 4 }
    );
}
