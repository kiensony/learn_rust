# task-305 — Report whether two labels differ after trimming

Compare two labels after trimming leading and trailing whitespace from each. Return true only if the remaining text differs. Preserve case and interior spaces. Add a regression test contrasting interior and outer whitespace.

```rust
pub fn needs_update(old: &str, new: &str) -> bool
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `needs_update("  blue", "blue")` gives `false`.
- `needs_update("East", "east")` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-305/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-305`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
