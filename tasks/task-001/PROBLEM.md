# task-001 — Greet

Return a greeting string.

```rust
pub fn greet(name: &str) -> String
```

- `greet("Ada")` → `"Hello, Ada!"`
- Empty name → `"Hello, stranger!"`
- Trim ASCII whitespace on `name` before use.
- Do not greet `" "` as a real name.

No `unsafe`. Std only.
