use task_1592::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_pages_total(&[]),
        if cfg!(feature = "wide") { 0 } else { 0 }
    );
    assert_eq!(
        feature_pages_total(&[4]),
        if cfg!(feature = "wide") { 128 } else { 4 }
    );
}
