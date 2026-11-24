use task_1566::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_padding_descending_steps(&[]),
        if cfg!(feature = "wide") { 0 } else { 0 }
    );
    assert_eq!(
        feature_padding_descending_steps(&[4]),
        if cfg!(feature = "wide") { 0 } else { 0 }
    );
}
