# task-043 — Choose the default binary

This package contains a customer screen and an operator screen. Set the package's `default-run` field to the customer binary `task_043`, so that `cargo run` from the task shows `Customer queue`. Explicitly selecting the operator binary must still show `Operator desk`.

From the repository directory, open `tasks/task-043/` and edit `Cargo.toml`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-043/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-043/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running the customer binary explicitly with `cargo run --bin task_043` produces standard output `"Customer queue\n"`.
- With the repaired manifest, `cargo run` selects the customer screen and prints "Customer queue\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-043/Cargo.toml
```
