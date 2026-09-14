# task-005 — Longest borrow

```rust
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str
```

- Return the longer slice (byte length). If equal length, return `a`.
- Do not allocate. Do not `to_string`.
- Lifetime must be explicit on the function as shown.

No `unsafe`.
