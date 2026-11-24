// rust-track: handwritten task-007
pub const TRACK_PROOF: u32 = 0x52535400 | 7;

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add(i64, i64),
    Sub(i64, i64),
    Div(i64, i64),
}

pub fn eval(op: Op) -> Result<i64, String> {
    match op {
        Op::Add(a, b) => a.checked_add(b),
        Op::Sub(a, b) => a.checked_sub(b),
        Op::Div(_, 0) => return Err("div by zero".into()),
        Op::Div(a, b) => a.checked_div(b),
    }
    .ok_or_else(|| String::from("overflow"))
}
