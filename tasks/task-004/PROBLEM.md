# task-004 — Own the label

Take ownership of a `String`, return a new owned label.

```rust
pub fn label(mut name: String, id: u32) -> String
```

- Result: `"{id}:{name}"` with `name` uppercased in ASCII only.
- `label(String::from("ada"), 7)` → `"7:ADA"`
- Reuse `name`’s buffer (`make_ascii_uppercase` + insert/push). Do not build from a fresh `format!` if you can avoid extra allocs — either is accepted if tests pass, but review prefers mutating `name`.
- Do not take `&str` and ignore the signature.

No `unsafe`.
