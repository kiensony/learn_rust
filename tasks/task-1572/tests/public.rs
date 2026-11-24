use task_1572::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_packing_total(&[]),
        if cfg!(feature = "wide") { 0 } else { 0 }
    );
    assert_eq!(
        feature_packing_total(&[4]),
        if cfg!(feature = "wide") { 32 } else { 4 }
    );
}
