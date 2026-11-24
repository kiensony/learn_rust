# task-1089 — Use an associated type · longest field with last tie (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Use the Packet trait’s associated Payload and return the selected byte length. Support every payload type implementing AsRef<str>.

```rust
pub fn longest_field_with_last_tie_equals(packet: &impl Packet) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `longest_field_with_last_tie_equals(&"72;12")` gives `2`.
- `longest_field_with_last_tie_equals(&String::from("猫=12;note"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1089/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1089`.

[Module reading](https://doc.rust-lang.org/reference/items/traits.html)
