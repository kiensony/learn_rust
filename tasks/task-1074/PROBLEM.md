# task-1074 — Use an associated type · closing marks removed (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Use the Packet trait’s associated Payload and return the selected byte length. Support every payload type implementing AsRef<str>.

```rust
pub fn closing_marks_removed_pipe(packet: &impl Packet) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `closing_marks_removed_pipe(&"first\n123")` gives `3`.
- `closing_marks_removed_pipe(&String::from("header\n猫|123"))` gives `7`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1074/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1074`.

[Module reading](https://doc.rust-lang.org/reference/items/traits.html)
