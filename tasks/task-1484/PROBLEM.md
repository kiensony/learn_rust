# task-1484 — Response boundaries · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Build the public decoder used by an external integration test. Accept ASCII whitespace-separated three-digit statuses from 100 through 599. Ignore malformed tokens. Observe indexes of successful or redirect responses (codes below 400) in the accepted sequence. Add a separate tests/consumer.rs that calls the crate as a downstream consumer and covers a malformed token among valid tokens.

Keep the first observation and every later one strictly above all earlier observations. Use an integration test outside src and the crate's public API. During review, point to the operation that enforces its contract.

```rust
pub fn response_boundaries_peaks(input: &str) -> Vec<u32>
```

## Public examples

- `response_boundaries_peaks("")` gives `vec![]`.
- `response_boundaries_peaks("200")` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1484/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1484`.

[Module reading](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
