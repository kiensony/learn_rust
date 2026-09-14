# task-007 — Op

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add(i64, i64),
    Sub(i64, i64),
    Div(i64, i64),
}

pub fn eval(op: Op) -> Result<i64, String>
```

- `Add` / `Sub` always succeed.
- `Div(_, 0)` → `Err("div by zero".into())`
- Integer division toward zero (Rust `/`).
- Overflow of `i64` add/sub → `Err("overflow".into())`. Use `checked_add` / `checked_sub`.

No `unsafe`. Match on `Op`.
