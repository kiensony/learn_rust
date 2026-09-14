#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add(i64, i64),
    Sub(i64, i64),
    Div(i64, i64),
}

pub fn eval(op: Op) -> Result<i64, String> {
    let _ = op;
    Ok(0)
}
