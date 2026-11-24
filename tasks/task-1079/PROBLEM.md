# task-1079 — Use an associated type · longest field with last tie (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Use the Packet trait’s associated Payload and return the selected byte length. Support every payload type implementing AsRef<str>.

```rust
pub fn longest_field_with_last_tie_pipe(packet: &impl Packet) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `longest_field_with_last_tie_pipe(&"first\n123")` gives `3`.
- `longest_field_with_last_tie_pipe(&String::from("|é|a"))` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1079/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1079`.

[Module reading](https://doc.rust-lang.org/reference/items/traits.html)
