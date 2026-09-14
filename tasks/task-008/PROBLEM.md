# task-008 — Word count

```rust
use std::collections::BTreeMap;

pub fn count_words(text: &str) -> BTreeMap<String, usize>
```

- Split on ASCII whitespace.
- Keys: lowercase ASCII (`make_ascii_lowercase` on a `String`).
- Ignore empty tokens.
- Values: occurrence counts.
- Use `BTreeMap` so order is sorted (tests compare maps, not print order).

`"A a b"` → `{ "a": 2, "b": 1 }`

No `unsafe`.
