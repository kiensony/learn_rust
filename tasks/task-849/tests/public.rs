use task_849::*;

#[test]
fn public_examples() {
    assert_eq!(dose_total(&[2, 4, 7]), Ok(13));
    assert_eq!(
        dose_total(&[2, -4, -2]),
        Err(DoseError {
            index: 1,
            value: -4
        })
    );
}
