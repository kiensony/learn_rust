# task-544 — Measure Unicode text · prefix before final mark (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. After selecting the field, count its Unicode scalar values. An emoji occupies one scalar value in the examples.

```rust
pub fn prefix_before_final_mark_equals(text: &str) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `prefix_before_final_mark_equals("72;12")` gives `0`.
- `prefix_before_final_mark_equals("猫=12;note")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-544/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-544`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
