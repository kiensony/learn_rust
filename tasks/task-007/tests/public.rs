use task_007::{eval, Op};

#[test]
fn add_sub() {
    assert_eq!(eval(Op::Add(2, 3)), Ok(5));
    assert_eq!(eval(Op::Sub(2, 3)), Ok(-1));
}

#[test]
fn div_zero() {
    assert_eq!(eval(Op::Div(1, 0)), Err("div by zero".into()));
}
