use task_1562::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_padding_total(&[]),
        if cfg!(feature = "wide") { 0 } else { 0 }
    );
    assert_eq!(
        feature_padding_total(&[4]),
        if cfg!(feature = "wide") { 16 } else { 4 }
    );
}
