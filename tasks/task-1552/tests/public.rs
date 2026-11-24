use task_1552::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_scaling_total(&[]),
        if cfg!(feature = "wide") { 0 } else { 0 }
    );
    assert_eq!(
        feature_scaling_total(&[4]),
        if cfg!(feature = "wide") { 8 } else { 4 }
    );
}
