# task-1080 — Use an associated type · shortest field with first tie (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Use the Packet trait’s associated Payload and return the selected byte length. Support every payload type implementing AsRef<str>.

```rust
pub fn shortest_field_with_first_tie_pipe(packet: &impl Packet) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `shortest_field_with_first_tie_pipe(&"first\n123")` gives `3`.
- `shortest_field_with_first_tie_pipe(&"hi|kit")` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1080/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1080`.

[Module reading](https://doc.rust-lang.org/reference/items/traits.html)
