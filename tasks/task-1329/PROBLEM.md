# task-1329 — Flag fields · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the odd parsed values.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use str::parse and explicit handling of parse errors. During review, point to the operation that enforces its contract.

```rust
pub fn flag_fields_longest_run(input: &str) -> u64
```

## Public examples

- `flag_fields_longest_run("")` gives `0`.
- `flag_fields_longest_run("1")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1329/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1329`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)
