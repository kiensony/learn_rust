# task-1494 — Status detail · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Build the public decoder used by an external integration test. Accept ASCII whitespace-separated three-digit statuses from 100 through 599. Ignore malformed tokens. Observe the final two decimal digits as a number. Add a separate tests/consumer.rs that calls the crate as a downstream consumer and covers a malformed token among valid tokens.

Keep the first observation and every later one strictly above all earlier observations. Use an integration test outside src and the crate's public API. During review, point to the operation that enforces its contract.

```rust
pub fn status_detail_peaks(input: &str) -> Vec<u32>
```

## Public examples

- `status_detail_peaks("")` gives `vec![]`.
- `status_detail_peaks("200")` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1494/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1494`.

[Module reading](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
