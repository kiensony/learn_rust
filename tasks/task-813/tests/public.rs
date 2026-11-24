use task_813::*;

#[test]
fn public_examples() {
    assert_eq!(reading(" 7 "), Ok(7));
    assert_eq!(reading("256"), Err(ReadingError::Range));
}
