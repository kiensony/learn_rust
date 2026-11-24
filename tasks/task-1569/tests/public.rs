use task_1569::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_padding_longest_run(&[]),
        if cfg!(feature = "wide") { 0 } else { 0 }
    );
    assert_eq!(
        feature_padding_longest_run(&[4]),
        if cfg!(feature = "wide") { 1 } else { 1 }
    );
}
