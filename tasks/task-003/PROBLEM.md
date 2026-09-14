# task-003 — Parse pair

Parse `"left:right"` into two `i32`s.

```rust
pub fn parse_pair(input: &str) -> Result<(i32, i32), String>
```

- One `:` separator. Trim each side.
- Success: `parse_pair(" 3 : -4 ")` → `Ok((3, -4))`
- No colon → `Err("missing colon".into())`
- More than one colon → `Err("too many colons".into())`
- Either side empty after trim → `Err("empty side".into())`
- Side not an `i32` → `Err("not an i32".into())`

Error strings must match exactly. No `unsafe`.
