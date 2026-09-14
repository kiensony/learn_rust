use task_007::{eval, Op, TRACK_PROOF};

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52535400 | 7);
}

#[test]
fn div_toward_zero() {
    assert_eq!(eval(Op::Div(-7, 3)), Ok(-2));
}

#[test]
fn overflow_add() {
    assert_eq!(eval(Op::Add(i64::MAX, 1)), Err("overflow".into()));
}

#[test]
fn overflow_sub() {
    assert_eq!(eval(Op::Sub(i64::MIN, 1)), Err("overflow".into()));
}
